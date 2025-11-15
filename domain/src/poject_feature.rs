use ogc::types::Feature;
use serde::Deserialize;
use serde_json::{Map, Value};

use crate::IntoOGCFeature;

#[derive(Deserialize)]
pub struct ProjectFeature {
    pub id: i32,
    pub properties: Map<String, Value>,
    pub geometry: geojson::Geometry,
}

impl IntoOGCFeature for ProjectFeature {
    fn into_ogc_feature(self, collection_url: String) -> ogc::types::Feature {
        let ProjectFeature {
            id,
            properties,
            geometry,
        } = self;
        Feature::new(id, collection_url)
            .set_geometry(geometry)
            .set_properties(properties)
    }
}
