use serde::Serialize;

use crate::{
    enums::{LineCapStyle, LineJoinStyle},
    layout::components::Color,
};

#[derive(Serialize)]
pub struct LayerOption {
    #[serde(rename = "@type")]
    option_type: String,
    #[serde(rename = "Option")]
    options: Vec<OptionItem>,
}

#[derive(Serialize)]
struct OptionItem {
    #[serde(rename = "@value")]
    value: String,
    #[serde(rename = "@type")]
    item_type: String,
    #[serde(rename = "@name")]
    name: String,
}

impl Default for LayerOption {
    fn default() -> Self {
        Self {
            option_type: "Map".to_string(),
            options: vec![
                OptionItem {
                    value: "3x:0,0,0,0,0,0".to_string(),
                    item_type: "QString".to_string(),
                    name: "border_width_map_unit_scale".to_string(),
                },
                OptionItem {
                    value: "no".to_string(),
                    item_type: "QString".to_string(),
                    name: "outline_style".to_string(),
                },
                OptionItem {
                    value: "255,255,255,255".to_string(),
                    item_type: "QString".to_string(),
                    name: "color".to_string(),
                },
                OptionItem {
                    value: "solid".to_string(),
                    item_type: "QString".to_string(),
                    name: "style".to_string(),
                },
            ],
        }
    }
}
impl LayerOption {
    pub fn fill_symbol(colour: Color) -> Self {
        Self {
            option_type: "Map".to_string(),
            options: vec![
                OptionItem {
                    name: "border_width_map_unit_scale".to_string(),
                    value: "3x:0,0,0,0,0,0".to_string(),
                    item_type: "QString".to_string(),
                },
                OptionItem {
                    name: "color".to_string(),
                    value: colour.to_string(),
                    item_type: "QString".to_string(),
                },
                OptionItem {
                    name: "joinstyle".to_string(),
                    value: "bevel".to_string(),
                    item_type: "QString".to_string(),
                },
                OptionItem {
                    name: "offset".to_string(),
                    value: "0,0".to_string(),
                    item_type: "QString".to_string(),
                },
                OptionItem {
                    name: "offset_map_unit_scale".to_string(),
                    value: "3x:0,0,0,0,0,0".to_string(),
                    item_type: "QString".to_string(),
                },
                OptionItem {
                    name: "offset_unit".to_string(),
                    value: "MM".to_string(),
                    item_type: "QString".to_string(),
                },
                OptionItem {
                    name: "outline_color".to_string(),
                    value: "35,35,35,255".to_string(),
                    item_type: "QString".to_string(),
                },
                OptionItem {
                    name: "outline_style".to_string(),
                    value: "no".to_string(),
                    item_type: "QString".to_string(),
                },
                OptionItem {
                    name: "outline_width".to_string(),
                    value: "0.26".to_string(),
                    item_type: "QString".to_string(),
                },
                OptionItem {
                    name: "outline_width_unit".to_string(),
                    value: "MM".to_string(),
                    item_type: "QString".to_string(),
                },
                OptionItem {
                    name: "style".to_string(),
                    value: "solid".to_string(),
                    item_type: "QString".to_string(),
                },
            ],
        }
    }

    pub fn line_symbol(join_style: LineJoinStyle, cap_style: LineCapStyle) -> Self {
        Self {
            option_type: "Map".to_string(),
            options: vec![
                OptionItem {
                    name: "joinstyle".to_string(),
                    value: join_style.to_string(),
                    item_type: "QString".to_string(),
                },
                OptionItem {
                    name: "align_dash_pattern".to_string(),
                    value: "0".to_string(),
                    item_type: "QString".to_string(),
                },
                OptionItem {
                    name: "capstyle".to_string(),
                    value: cap_style.to_string(),
                    item_type: "QString".to_string(),
                },
                OptionItem {
                    name: "customdash".to_string(),
                    value: "5;2".to_string(),
                    item_type: "QString".to_string(),
                },
                OptionItem {
                    name: "customdash_map_unit_scale".to_string(),
                    value: "3x:0,0,0,0,0,0".to_string(),
                    item_type: "QString".to_string(),
                },
                OptionItem {
                    name: "customdash_unit".to_string(),
                    value: "MM".to_string(),
                    item_type: "QString".to_string(),
                },
                OptionItem {
                    name: "dash_pattern_offset".to_string(),
                    value: "0".to_string(),
                    item_type: "QString".to_string(),
                },
                OptionItem {
                    name: "dash_pattern_offset_map_unit_scale".to_string(),
                    value: "3x:0,0,0,0,0,0".to_string(),
                    item_type: "QString".to_string(),
                },
                OptionItem {
                    name: "dash_pattern_offset_unit".to_string(),
                    value: "MM".to_string(),
                    item_type: "QString".to_string(),
                },
                OptionItem {
                    name: "draw_inside_polygon".to_string(),
                    value: "0".to_string(),
                    item_type: "QString".to_string(),
                },
                OptionItem {
                    name: "line_color".to_string(),
                    value: "0,0,0,255".to_string(),
                    item_type: "QString".to_string(),
                },
                OptionItem {
                    name: "line_style".to_string(),
                    value: "solid".to_string(),
                    item_type: "QString".to_string(),
                },
                OptionItem {
                    name: "line_width".to_string(),
                    value: "0.3".to_string(),
                    item_type: "QString".to_string(),
                },
                OptionItem {
                    name: "line_width_unit".to_string(),
                    value: "MM".to_string(),
                    item_type: "QString".to_string(),
                },
                OptionItem {
                    name: "offset".to_string(),
                    value: "0".to_string(),
                    item_type: "QString".to_string(),
                },
                OptionItem {
                    name: "offset_map_unit_scale".to_string(),
                    value: "3x:0,0,0,0,0,0".to_string(),
                    item_type: "QString".to_string(),
                },
                OptionItem {
                    name: "offset_unit".to_string(),
                    value: "MM".to_string(),
                    item_type: "QString".to_string(),
                },
                OptionItem {
                    name: "ring_filter".to_string(),
                    value: "0".to_string(),
                    item_type: "QString".to_string(),
                },
                OptionItem {
                    name: "trim_distance_end".to_string(),
                    value: "0".to_string(),
                    item_type: "QString".to_string(),
                },
                OptionItem {
                    name: "trim_distance_end_map_unit_scale".to_string(),
                    value: "3x:0,0,0,0,0,0".to_string(),
                    item_type: "QString".to_string(),
                },
                OptionItem {
                    name: "trim_distance_end_unit".to_string(),
                    value: "MM".to_string(),
                    item_type: "QString".to_string(),
                },
                OptionItem {
                    name: "trim_distance_start".to_string(),
                    value: "0".to_string(),
                    item_type: "QString".to_string(),
                },
                OptionItem {
                    name: "trim_distance_start_map_unit_scale".to_string(),
                    value: "3x:0,0,0,0,0,0".to_string(),
                    item_type: "QString".to_string(),
                },
                OptionItem {
                    name: "trim_distance_start_unit".to_string(),
                    value: "MM".to_string(),
                    item_type: "QString".to_string(),
                },
                OptionItem {
                    name: "tweak_dash_pattern_on_corners".to_string(),
                    value: "0".to_string(),
                    item_type: "QString".to_string(),
                },
                OptionItem {
                    name: "use_custom_dash".to_string(),
                    value: "0".to_string(),
                    item_type: "QString".to_string(),
                },
                OptionItem {
                    name: "width_map_unit_scale".to_string(),
                    value: "3x:0,0,0,0,0,0".to_string(),
                    item_type: "QString".to_string(),
                },
            ],
        }
    }

    pub fn marker_symbol() -> Self {
        Self {
            option_type: "Map".into(),
            options: vec![
                OptionItem {
                    name: "angle".to_string(),
                    value: "0".to_string(),
                    item_type: "QString".to_string(),
                },
                OptionItem {
                    name: "cap_style".to_string(),
                    value: LineCapStyle::Square.to_string(),
                    item_type: "QString".to_string(),
                },
                OptionItem {
                    name: "color".to_string(),
                    value: "0,0,0,255".to_string(),
                    item_type: "QString".to_string(),
                },
                OptionItem {
                    name: "horizontal_anchor_point".to_string(),
                    value: "1".to_string(),
                    item_type: "QString".to_string(),
                },
                OptionItem {
                    name: "joinstyle".to_string(),
                    value: "bevel".to_string(),
                    item_type: "QString".to_string(),
                },
                OptionItem {
                    name: "name".to_string(),
                    value: "circle".to_string(),
                    item_type: "QString".to_string(),
                },
                OptionItem {
                    name: "offset".to_string(),
                    value: "0,0".to_string(),
                    item_type: "QString".to_string(),
                },
                OptionItem {
                    name: "offset_map_unit_scale".to_string(),
                    value: "3x:0,0,0,0,0,0".to_string(),
                    item_type: "QString".to_string(),
                },
                OptionItem {
                    name: "offset_unit".to_string(),
                    value: "MM".to_string(),
                    item_type: "QString".to_string(),
                },
                OptionItem {
                    name: "outline_color".to_string(),
                    value: "35,35,35,255".to_string(),
                    item_type: "QString".to_string(),
                },
                OptionItem {
                    name: "outline_style".to_string(),
                    value: "solid".to_string(),
                    item_type: "QString".to_string(),
                },
                OptionItem {
                    name: "outline_width".to_string(),
                    value: "0".to_string(),
                    item_type: "QString".to_string(),
                },
                OptionItem {
                    name: "outline_width_map_unit_scale".to_string(),
                    value: "3x:0,0,0,0,0,0".to_string(),
                    item_type: "QString".to_string(),
                },
                OptionItem {
                    name: "outline_width_unit".to_string(),
                    value: "MM".to_string(),
                    item_type: "QString".to_string(),
                },
                OptionItem {
                    name: "scale_method".to_string(),
                    value: "diameter".to_string(),
                    item_type: "QString".to_string(),
                },
                OptionItem {
                    name: "size".to_string(),
                    value: "2".to_string(),
                    item_type: "QString".to_string(),
                },
                OptionItem {
                    name: "size_map_unit_scale".to_string(),
                    value: "3x:0,0,0,0,0,0".to_string(),
                    item_type: "QString".to_string(),
                },
                OptionItem {
                    name: "size_unit".to_string(),
                    value: "MM".to_string(),
                    item_type: "QString".to_string(),
                },
                OptionItem {
                    name: "vertical_anchor_point".to_string(),
                    value: "1".to_string(),
                    item_type: "QString".to_string(),
                },
            ],
        }
    }
}
