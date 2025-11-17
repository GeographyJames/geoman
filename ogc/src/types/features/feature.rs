use crate::types::common::{
    Link, MediaType,
    link_relations::{COLLECTION, SELF},
};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Serialize, Default, Deserialize)]
pub enum Type {
    #[default]
    Feature,
}

#[non_exhaustive]
#[derive(Serialize, Deserialize)]
pub struct Feature {
    pub id: i32,
    pub r#type: Type,
    pub properties: Map<String, Value>,
    pub geometry: Option<geojson::Geometry>,
    pub links: [Link; 2],
}

impl Feature {
    pub fn new(id: i32, collection_url: String) -> Self {
        Self {
            id,
            links: [
                Link::new(format!("{collection_url}/items/{id}"), SELF)
                    .mediatype(MediaType::GeoJson),
                Link::new(collection_url, COLLECTION).mediatype(MediaType::Json),
            ],
            r#type: Default::default(),
            properties: Default::default(),
            geometry: Default::default(),
        }
    }

    pub fn set_properties(mut self, properties: Map<String, Value>) -> Self {
        self.properties = properties;
        self
    }

    pub fn append_properties(mut self, properties: &mut Map<String, Value>) -> Self {
        self.properties.append(properties);
        self
    }

    pub fn set_geometry(mut self, geometry: geojson::Geometry) -> Self {
        self.geometry = Some(geometry);
        self
    }
    pub fn insert_property(mut self, key: String, value: serde_json::Value) -> Self {
        self.properties.insert(key, value);
        self
    }

    /// Removes the key from the feature properties and returns the value if the key
    /// is in the feature properties. Fails is the value is not a string.
    pub fn remove_string_property(
        &mut self,
        key: &str,
    ) -> Option<Result<String, serde_json::Error>> {
        self.properties.remove(key).map(serde_json::from_value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn feature_serialises_and_deserialises_to_and_from_geojson() {
        let feature = Feature::new(Default::default(), uuid::Uuid::new_v4().to_string());
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
