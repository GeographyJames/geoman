use ogcapi_types::common::{
    Link,
    link_rel::{COLLECTION, SELF},
    media_type::{GEO_JSON, JSON},
};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
#[derive(Serialize, Default, Deserialize, Clone)]
pub enum Type {
    #[default]
    Feature,
}
#[non_exhaustive]
#[derive(Serialize, Deserialize, Clone)]
pub struct Feature {
    pub id: i32,
    pub r#type: Type,
    pub properties: Map<String, Value>,
    pub geometry: Option<geojson::Geometry>,
    pub links: [Link; 2],
}

impl Feature {
    pub fn new(
        id: i32,
        properties: Map<String, Value>,
        geometry: Option<geojson::Geometry>,
        collection_url: String,
    ) -> Self {
        let links = [
            Link::new(format!("{collection_url}/items/{id}"), SELF).mediatype(GEO_JSON),
            Link::new(collection_url, COLLECTION).mediatype(JSON),
        ];
        Self {
            id,
            r#type: Default::default(),
            properties,
            geometry,
            links,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn feature_serialises_and_deserialises_to_and_from_geojson() {
        let feature = Feature {
            id: 0,
            r#type: Default::default(),
            properties: Default::default(),
            geometry: None,
            links: [Link::new("href", "rel"), Link::new("href", "rel")],
        };
        let json = json!(feature);
        let geojson: geojson::Feature =
            serde_json::from_value(json).expect("failed to deserialise to geojson feature");
        check_geojson(&geojson);
        let json = json!(geojson);
        let _feature: Feature =
            serde_json::from_value(json).expect("fialed to deserialise to ogc feature");
    }

    /// Asserts a GeoJson feature matches required criteria
    fn check_geojson(geojson: &geojson::Feature) {
        // Verify the feature has two links
        let links = geojson
            .foreign_members
            .as_ref()
            .expect("no foreign members")
            .get("links")
            .expect("feature has no links");
        match links {
            serde_json::Value::Array(values) => {
                let links: Vec<Link> = values
                    .into_iter()
                    .map(|v| {
                        serde_json::from_value(v.clone()).expect("failed to deserialise links")
                    })
                    .collect();
                assert_eq!(links.len(), 2, "feature should have 2 links")
            }
            _ => panic!("links is not an array"),
        }

        // Verify the feature has integer id
        let id = geojson.id.as_ref().expect("feature has no id");

        match id {
            geojson::feature::Id::Number(number) => {
                let _id_value: i32 = number
                    .as_i64()
                    .expect("feature id is not a valid i64")
                    .try_into()
                    .expect("feature id is not valid i32");
            }
            geojson::feature::Id::String(_) => panic!("feature id not a number"),
        }
    }
}
