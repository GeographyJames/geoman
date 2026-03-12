use serde::Serialize;

use crate::qgis::Symbol;
mod composer_map_overview;
mod map_grid;
pub use composer_map_overview::ComposerMapOverview;
pub use map_grid::ComposerMapGrid;
#[derive(Serialize)]
pub struct AtlasMap {
    #[serde(rename = "@atlasDriven")]
    atlas_driven: u32,
    #[serde(rename = "@margin")]
    margin: f64,
    #[serde(rename = "@scalingMode")]
    scaling_mode: u32,
}

impl Default for AtlasMap {
    fn default() -> Self {
        Self {
            atlas_driven: Default::default(),
            margin: 0.1,
            scaling_mode: 2,
        }
    }
}

#[derive(Serialize)]
pub struct LayoutItemSymbol {
    pub symbol: Symbol,
}

#[derive(Serialize, Default)]
pub struct LabelBlockingItems {}

#[derive(Serialize)]
pub struct NumericFormat {
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "Option")]
    option: NumericFormatOption,
}

#[derive(Serialize)]
pub struct NumericFormatOption {
    #[serde(rename = "@type")]
    option_type: String,
    #[serde(rename = "Option")]
    options: Vec<NumericFormatSubOption>,
}

#[derive(Serialize)]
pub struct NumericFormatSubOption {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    option_type: Option<String>,
    #[serde(rename = "@value", skip_serializing_if = "Option::is_none")]
    value: Option<String>,
}

impl Default for NumericFormat {
    fn default() -> Self {
        Self {
            id: "basic".to_string(),
            option: NumericFormatOption {
                option_type: "Map".to_string(),
                options: vec![
                    NumericFormatSubOption {
                        name: "decimal_separator".to_string(),
                        option_type: Some("invalid".to_string()),
                        value: None,
                    },
                    NumericFormatSubOption {
                        name: "decimals".to_string(),
                        option_type: Some("int".to_string()),
                        value: Some("6".to_string()),
                    },
                    NumericFormatSubOption {
                        name: "rounding_type".to_string(),
                        option_type: Some("int".to_string()),
                        value: Some("0".to_string()),
                    },
                    NumericFormatSubOption {
                        name: "show_plus".to_string(),
                        option_type: Some("bool".to_string()),
                        value: Some("false".to_string()),
                    },
                    NumericFormatSubOption {
                        name: "show_thousand_separator".to_string(),
                        option_type: Some("bool".to_string()),
                        value: Some("true".to_string()),
                    },
                    NumericFormatSubOption {
                        name: "show_trailing_zeros".to_string(),
                        option_type: Some("bool".to_string()),
                        value: Some("false".to_string()),
                    },
                    NumericFormatSubOption {
                        name: "thousand_separator".to_string(),
                        option_type: Some("invalid".to_string()),
                        value: None,
                    },
                ],
            },
        }
    }
}

#[derive(Serialize, Default)]
pub struct LayerSet {}
