use serde::Serialize;

use crate::qgis::layer::DataProvider;

#[derive(Serialize, Default, Clone)]
pub struct AuxiliaryLayer {}

#[derive(Serialize, Default, Clone)]
pub struct MetadataUrls {}

#[derive(Serialize, Clone)]
pub struct Flags {
    #[serde(rename = "Identifiable")]
    pub identifiable: u8,
    #[serde(rename = "Removable")]
    pub removable: u8,
    #[serde(rename = "Searchable")]
    pub searchable: u8,
    #[serde(rename = "Private")]
    pub private: u8,
}

impl Default for Flags {
    fn default() -> Self {
        Self {
            identifiable: 1,
            removable: 1,
            searchable: 1,
            private: 0,
        }
    }
}

#[derive(Serialize, Default, Clone)]
pub struct KeywordList {
    pub value: String,
}
#[allow(non_snake_case)]
#[derive(Serialize, Default, Clone)]
pub struct NoData {
    pub noDataList: NoDataList,
}

#[derive(Serialize, Clone)]
pub struct NoDataList {
    #[serde(rename = "@useSrcNoData")]
    pub use_src_no_data: u16,
    #[serde(rename = "@bandNo")]
    pub band_no: u16,
}

impl Default for NoDataList {
    fn default() -> Self {
        Self {
            use_src_no_data: Default::default(),
            band_no: 1,
        }
    }
}

#[derive(Serialize, Default, Clone)]
pub struct Provider {
    #[serde(rename = "@encoding", skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,
    #[serde(rename = "$text")]
    pub text: DataProvider,
}

#[derive(Serialize, Default, Clone)]
pub struct VectorJoins {}

#[derive(Serialize, Default, Clone)]
pub struct LayerDependencies {}

#[derive(Serialize, Default, Clone)]
pub struct DataDependencies {}

#[derive(Serialize, Default, Clone)]
pub struct ExpressionFields {}

#[allow(non_snake_case)]
#[derive(Serialize, Default, Clone)]
pub struct Temporal {
    #[serde(rename = "@enabled")]
    pub enabled: u16,
    #[serde(rename = "@fetchMode")]
    pub fetch_mode: u16,
    #[serde(rename = "@mode")]
    pub mode: u16,
    pub fixedRange: FixedRange,
}

#[derive(Serialize, Default, Clone)]
pub struct FixedRange {
    pub start: String,
    pub end: String,
}
