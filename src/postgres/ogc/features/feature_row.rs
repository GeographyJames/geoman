use serde::Deserialize;
use serde_json::{Map, Value};

#[derive(Deserialize)]
pub struct FeatureRow {
    pub id: i32,
    pub properties: Map<String, Value>,
    pub geometry: geojson::Geometry,
}
