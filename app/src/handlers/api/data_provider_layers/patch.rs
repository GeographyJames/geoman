use domain::enums::LayerCategory;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize, Default)]
pub struct DataProviderLayerUpdatePayload {
    pub name: Option<String>,
    pub abbreviation: Option<String>,
    pub source: Option<Value>,
    pub category: Option<LayerCategory>,
    pub description: Option<String>,
    pub enabled: Option<bool>,
    pub style_config: Option<Value>,
    pub display_options: Option<Value>,
    pub country_code: Option<String>,
    pub subdivision: Option<String>,
    pub sort_order: Option<i32>,
}
