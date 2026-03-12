use qgis::enums::ScalebarUnits;
use serde::{Deserialize, Serialize};

use crate::features::figure_tool::CopyrightText;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct FigureProperties {
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub extra_legend_text: Option<String>,
    pub enable_html: Option<bool>,
    pub figure_number: Option<String>,
    pub scalebar_units: Option<ScalebarUnits>,
    pub scalebar_units_per_segment: Option<u32>,
    pub logo: Option<bool>,
    pub internal_use: Option<bool>,
    pub map_ticks: Option<bool>,
    pub north_arrow: Option<bool>,
    pub status: Option<String>,
    pub target_x_coordinate: Option<f64>,
    pub target_y_coordinate: Option<f64>,
    pub scalebar: Option<bool>,
    pub copyright_text: Option<CopyrightText>,
    pub custom_copyright_text: Option<String>,
    pub greyscale_background_map: Option<bool>,
    pub legend_text_vmargin: Option<u32>,
    pub legend: Option<bool>,
    pub grid_spacing: Option<u32>,
    pub legend_height_percent: Option<u32>,
    pub overview_frame: Option<bool>,
    pub overview_map_scale: Option<u32>,
}
