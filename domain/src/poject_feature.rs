use ogc::types::Feature;
use serde::Deserialize;
use serde_json::{Map, Value, json};

use crate::IntoOGCFeature;

#[derive(Deserialize)]
pub struct ProjectFeature {
    pub id: i32,
    pub properties: Map<String, Value>,
    pub name: String,
    pub geometry: geojson::Geometry,
    pub is_primary: bool,
}

impl IntoOGCFeature for ProjectFeature {
    fn into_ogc_feature(self, collection_url: String) -> ogc::types::Feature {
        let ProjectFeature {
            id,
            properties,
            geometry,
            name,
            is_primary,
        } = self;
        Feature::new(id, collection_url)
            .set_geometry(geometry)
            .set_properties(properties)
            .insert_property("name".to_string(), json!(name))
            .insert_property("is_primary".to_string(), json!(is_primary))
    }
}
