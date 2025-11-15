use crate::types::{
    Feature,
    common::{Link, MediaType, link_relations::SELF},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Default, Deserialize)]
pub enum Type {
    #[default]
    FeatureCollection,
}

#[non_exhaustive]
#[derive(Serialize, Deserialize)]
pub struct FeatureCollection {
    pub id: String,
    pub r#type: Type,
    pub features: Vec<Feature>,
    pub links: [Link; 1],
}

impl FeatureCollection {
    pub fn new(collection_url: String, slug: String) -> Self {
        Self {
            id: slug,
            r#type: Type::default(),
            features: Default::default(),
            links: [
                Link::new(format!("{}/items", collection_url), SELF).mediatype(MediaType::GeoJson)
            ],
        }
    }
    pub fn append_features(mut self, mut features: Vec<Feature>) -> Self {
        self.features.append(&mut features);
        self
    }

    pub fn opening_json(&self) -> Result<String, serde_json::Error> {
        Ok(format!(
            r#"{{"type":{},"id":{},"links":{},"features":["#,
            serde_json::to_string(&self.r#type)?,
            serde_json::to_string(&self.id)?,
            serde_json::to_string(&self.links)?
        ))
    }
    pub fn closing_json(&self) -> String {
        "]}".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    impl Default for FeatureCollection {
        fn default() -> Self {
            Self::new(
                uuid::Uuid::new_v4().to_string(),
                uuid::Uuid::new_v4().to_string(),
            )
        }
    }

    #[test]
    fn feature_collection_serialises_to_and_from_geojson() {
        let feature_collection = FeatureCollection::default();
        let json = json!(feature_collection);
        let geojson: geojson::FeatureCollection = serde_json::from_value(json)
            .expect("failed to deserialize to geojson feature collection");
        check_geojson(&geojson);
        let json = json!(geojson);
        let _feature_collection: FeatureCollection =
            serde_json::from_value(json).expect("failed to deserialise to ogc feature collection");
    }

    #[test]
    fn feature_collection_opening_and_closing_json_serialises_to_geojson_and_deserialises_to_feature_collection()
     {
        let fc = FeatureCollection::default();
        let json_string = format!(
            "{}{}",
            fc.opening_json().expect("failed to serialise opening json"),
            fc.closing_json()
        );
        let _: FeatureCollection = serde_json::from_str(&json_string)
            .expect("failed to deserialise to feature collection");
    }

    fn check_geojson(geojson: &geojson::FeatureCollection) {
        let foreign_members = geojson
            .foreign_members
            .as_ref()
            .expect("collection has no foreign members");

        // Verifyt collection has a link
        let links = foreign_members
            .get("links")
            .expect("collection has no links");
        match links {
            serde_json::Value::Array(values) => {
                let links: Vec<Link> = values
                    .into_iter()
                    .map(|v| {
                        serde_json::from_value(v.clone()).expect("failed to deserialise links")
                    })
                    .collect();
                assert_eq!(links.len(), 1, "collection should have 1 link")
            }
            _ => panic!("links is not an array"),
        }

        // Verify collection has a string id
        let id = foreign_members.get("id").expect("collection has no id");
        match id {
            serde_json::Value::String(_) => {}
            _ => panic!("id is not a string"),
        }
    }
}
