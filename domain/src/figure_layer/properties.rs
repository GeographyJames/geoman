use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct LayerProperties {
    pub legend_text: Option<String>,
    pub convert_boundary_to_singleparts: bool,
    pub include_on_map: bool,
    pub include_on_legend: bool,
    pub include_as_target: bool,
    pub enable_labels: bool,
}
