mod layer_option;
pub use layer_option::LayerOption;

use serde::Serialize;

use crate::{
    data_defined_properties::{DataDefinedProperties, PropertyOption},
    enums::{LineCapStyle, LineJoinStyle},
    layout::components::Color,
};

#[derive(Serialize)]
pub struct Symbol {
    #[serde(rename = "@clip_to_extent")]
    clip_to_extent: String,
    #[serde(rename = "@type")]
    pub symbol_type: String,
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@force_rhr")]
    force_rhr: String,
    #[serde(rename = "@is_animated")]
    is_animated: String,
    #[serde(rename = "@alpha")]
    alpha: String,
    #[serde(rename = "@frame_rate")]
    frame_rate: String,
    pub data_defined_properties: DataDefinedProperties,
    pub layer: SymbolLayer,
}

impl Default for Symbol {
    fn default() -> Self {
        Self {
            clip_to_extent: "1".to_string(),
            symbol_type: "fill".to_string(),
            name: "".to_string(),
            force_rhr: "0".to_string(),
            is_animated: "0".to_string(),
            alpha: "1".to_string(),
            frame_rate: "10".to_string(),
            data_defined_properties: DataDefinedProperties::default(),
            layer: SymbolLayer::default(),
        }
    }
}

#[derive(Serialize)]
pub struct SymbolLayer {
    #[serde(rename = "@class")]
    pub class: String,
    #[serde(rename = "@enabled")]
    enabled: String,
    #[serde(rename = "@locked")]
    locked: String,
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "@pass")]
    pass: String,
    #[serde(rename = "Option")]
    option: LayerOption,
    data_defined_properties: DataDefinedProperties,
}

impl Default for SymbolLayer {
    fn default() -> Self {
        Self {
            class: "SimpleFill".to_string(),
            enabled: "1".to_string(),
            locked: "0".to_string(),
            id: format!("{{{}}}", uuid::Uuid::new_v4()),
            pass: "0".to_string(),
            option: LayerOption::default(),
            data_defined_properties: DataDefinedProperties::default(),
        }
    }
}

impl Symbol {
    pub fn marker_symbol() -> Self {
        let symbol_layer = SymbolLayer {
            class: "SimpleMarker".into(),
            option: LayerOption::marker_symbol(),
            data_defined_properties: DataDefinedProperties::new_with_extra_option(),
            ..Default::default()
        };
        Self {
            symbol_type: "marker".into(),
            data_defined_properties: DataDefinedProperties::new_with_extra_option(),
            layer: symbol_layer,

            ..Default::default()
        }
    }
    pub fn line_symbol(join_style: LineJoinStyle, cap_style: LineCapStyle) -> Self {
        let mut symbol = Self::default();
        let extra_option = PropertyOption {
            value: None,
            prop_type: None,
            name: "properties".into(),
        };
        symbol.symbol_type = "line".into();
        symbol.layer.class = "SimpleLine".into();
        symbol.layer.option = LayerOption::line_symbol(join_style, cap_style);
        symbol
            .data_defined_properties
            .option
            .options
            .push(extra_option.clone());
        symbol
            .layer
            .data_defined_properties
            .option
            .options
            .push(extra_option);
        symbol
    }

    pub fn fill_symbol(colour: Color) -> Self {
        let extra_option = PropertyOption {
            value: None,
            prop_type: None,

            name: "properties".into(),
        };
        let mut symbol = Self::default();
        symbol.layer.option = LayerOption::fill_symbol(colour);

        symbol
            .data_defined_properties
            .option
            .options
            .push(extra_option.clone());
        symbol
            .layer
            .data_defined_properties
            .option
            .options
            .push(extra_option);
        symbol
    }
}
