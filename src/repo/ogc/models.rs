use serde::Deserialize;
use serde_json::{Map, Value};

pub struct CollectionRow {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct FeatureRow {
    pub id: i32,
    pub properties: Map<String, Value>,
    pub geometry: geojson::Geometry,
}
