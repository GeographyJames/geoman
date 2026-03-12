use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum BaseMapDataSource {
    WMTS(WMTSDataSource),
    WMS(WMSDataSource),
    XYZ(XYZDataSource),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WMSDataSource {
    pub url: String,
    pub alt_project_download_url: Option<String>,
    pub epsg_id: u16,
    pub authcfg_id: Option<String>,
    pub layers: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct XYZDataSource {
    pub url: String,
    pub epsg_id: u16,
    pub authcfg_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WMTSDataSource {
    pub url: String,
    pub alt_project_download_url: Option<String>,
    pub layers: String,
    pub tile_matrix_set: String,
    pub epsg_id: u16,
    pub authcfg_id: Option<String>,
}

impl BaseMapDataSource {
    pub fn set_url_to_alt_url(&mut self) {
        match self {
            BaseMapDataSource::WMS(src) => {
                if let Some(ref url) = src.alt_project_download_url {
                    src.url = url.to_owned()
                }
            }
            BaseMapDataSource::WMTS(src) => {
                if let Some(ref url) = src.alt_project_download_url {
                    src.url = url.to_owned()
                }
            }
            _ => {}
        }
    }
    pub fn epsg_id(&self) -> u16 {
        match self {
            BaseMapDataSource::WMS(src) => src.epsg_id,
            BaseMapDataSource::XYZ(src) => src.epsg_id,
            BaseMapDataSource::WMTS(src) => src.epsg_id,
        }
    }
}
