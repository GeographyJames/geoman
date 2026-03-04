use serde::{Deserialize, Serialize};

use crate::{
    DataProviderId, DataProviderLayerId, DataProviderServiceId,
    enums::{DataProviderServiceType, LayerCategory},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct DataProvider {
    pub id: DataProviderId,
    pub name: String,
    pub description: Option<String>,
    pub country_code: Option<String>,
    pub subdivision: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DataProviderService {
    pub id: DataProviderServiceId,
    pub provider_id: DataProviderId,
    pub name: String,
    pub service_type: DataProviderServiceType,
    pub base_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DataProviderLayer {
    pub id: DataProviderLayerId,
    pub service_id: DataProviderServiceId,
    pub name: String,
    pub abbreviation: Option<String>,
    pub source: serde_json::Value,
    pub category: LayerCategory,
    pub description: Option<String>,
    pub enabled: bool,
    pub style_config: serde_json::Value,
    pub display_options: serde_json::Value,
    pub country_code: Option<String>,
    pub subdivision: Option<String>,
    pub sort_order: i32,
}
