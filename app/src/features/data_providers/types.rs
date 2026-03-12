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
    #[sqlx(rename = "WMTS")]
    Wmts,
    #[sqlx(rename = "WFS")]
    Wfs,
    ArcGISRest,
    #[sqlx(rename = "MVT")]
    Mvt,
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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum LayerSource {
    ImageWms {
        url: String,
        layers: String,
        epsg_id: u16,
        alt_project_download_url: Option<String>,
        authcfg_id: Option<String>,
    },
    TileWms {
        url: String,
        layers: String,
        epsg_id: u16,
        alt_project_download_url: Option<String>,
        authcfg_id: Option<String>,
    },
    Wmts {
        url: String,
        layers: String,
        tile_matrix_set: String,
        epsg_id: u16,
        alt_project_download_url: Option<String>,
        authcfg_id: Option<String>,
    },
    Xyz {
        url: String,
        epsg_id: u16,
        authcfg_id: Option<String>,
    },
    Mvt {
        url: String,
    },
    #[serde(rename = "arcgis_rest")]
    ArcGisRest {
        service_name: String,
        layer_id: i64,
        name_field: Option<String>,
    },
    Wfs {
        url: String,
        layer_name: String,
        name_field: Option<String>,
    },
    OgcApiFeatures {
        url: String,
        collection_name: String,
    },
}

impl Default for LayerSource {
    fn default() -> Self {
        Self::Mvt { url: String::new() }
    }
}

impl LayerSource {
    pub fn epsg_id(&self) -> Option<u16> {
        match self {
            LayerSource::ImageWms { epsg_id, .. } => Some(*epsg_id),
            LayerSource::TileWms { epsg_id, .. } => Some(*epsg_id),
            LayerSource::Wmts { epsg_id, .. } => Some(*epsg_id),
            LayerSource::Xyz { epsg_id, .. } => Some(*epsg_id),
            _ => None,
        }
    }

    pub fn set_url_to_alt_url(&mut self) {
        match self {
            LayerSource::ImageWms { url, alt_project_download_url, .. }
            | LayerSource::TileWms { url, alt_project_download_url, .. }
            | LayerSource::Wmts { url, alt_project_download_url, .. } => {
                if let Some(alt) = alt_project_download_url.take() {
                    *url = alt;
                }
            }
            _ => {}
        }
    }
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
    pub source: Option<sqlx::types::Json<LayerSource>>,
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
