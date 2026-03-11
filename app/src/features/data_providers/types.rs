use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Clone, Copy, Default, Debug, Deserialize, Serialize, sqlx::Type, PartialEq)]
#[sqlx(transparent)]
pub struct DataProviderId(pub i32);

#[derive(Clone, Copy, Default, Debug, Deserialize, Serialize, sqlx::Type, PartialEq)]
#[sqlx(transparent)]
pub struct DataProviderServiceId(pub i32);

#[derive(Clone, Copy, Default, Debug, Deserialize, Serialize, sqlx::Type, PartialEq)]
#[sqlx(transparent)]
pub struct DataProviderLayerId(pub i32);

#[derive(Serialize, Deserialize, Debug, Type, Clone, PartialEq, Default)]
#[sqlx(type_name = "app.data_provider_service_type")]
pub enum DataProviderServiceType {
    #[default]
    ImageWMS,
    TileWMS,
    WMTS,
    WFS,
    ArcGISRest,
    MVT,
    OGCAPIFeatures,
    XYZ,
}

#[derive(Serialize, Deserialize, Debug, Type, Clone, PartialEq)]
#[sqlx(type_name = "app.layer_category", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum LayerCategory {
    Overlay,
    Basemap,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DataProvider {
    pub id: DataProviderId,
    pub name: String,
    pub country_code: Option<String>,
    pub subdivision: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DataProviderService {
    pub id: DataProviderServiceId,
    pub provider_id: DataProviderId,
    pub name: String,
    pub service_type: DataProviderServiceType,
    pub base_url: Option<String>,
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
    pub enabled_geoman: bool,
    pub enabled_figure_tool: bool,
    pub style_config: serde_json::Value,
    pub display_options: serde_json::Value,
    pub country_code: Option<String>,
    pub subdivision: Option<String>,
    pub sort_order: i32,
}
