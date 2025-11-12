use serde::Deserialize;
use serde_json::{Map, Value};

pub struct DbQueryParams {
    pub limit: Option<i64>,
}

#[derive(Deserialize)]
pub struct FeatureRow {
    pub id: i32,
    pub properties: Map<String, Value>,
    pub geometry: geojson::Geometry,
}
