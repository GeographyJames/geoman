use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    ogc::types::{
        Feature,
        common::{Link, link_relations::SELF, media_types::MediaType},
    },
    repo::ogc::FeatureRow,
};

#[derive(Serialize, Default, Deserialize)]
pub enum Type {
    #[default]
    FeatureCollection,
}

#[derive(Serialize, Deserialize)]
pub struct FeatureCollection {
    pub id: String,
    pub r#type: Type,
    pub features: Vec<Feature>,
    pub links: [Link; 1],
}

impl FeatureCollection {
    pub fn from_feature_rows(rows: Vec<FeatureRow>, collection_url: String, slug: String) -> Self {
        Self {
            id: slug,
            r#type: Type::default(),
            features: rows
                .into_iter()
                .map(|f| Feature::from_feature_row(f, collection_url.clone()))
                .collect(),
            links: [
                Link::new(format!("{}/items", collection_url), SELF).mediatype(MediaType::GeoJson)
            ],
        }
    }
}

impl Default for FeatureCollection {
    fn default() -> Self {
        Self::from_feature_rows(
            Vec::default(),
            Uuid::new_v4().to_string(),
            Uuid::new_v4().to_string(),
        )
    }
}

#[cfg(test)]
mod tests {

    use crate::ogc::types::{FeatureCollection, common::Link};
    use serde_json::json;

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
