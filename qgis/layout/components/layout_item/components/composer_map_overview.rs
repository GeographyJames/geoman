use serde::Serialize;

use crate::qgis::{
    QgisUuid,
    layout::components::{Color, LayoutObject},
    symbol::Symbol,
};

#[derive(Serialize)]
pub struct ComposerMapOverview {
    #[serde(rename = "@position")]
    pub position: u32,
    #[serde(rename = "@inverted")]
    pub inverted: u32,
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@centered")]
    pub centered: u32,
    #[serde(rename = "@uuid")]
    pub uuid: QgisUuid,
    #[serde(rename = "@frameMap")]
    pub frame_map: QgisUuid,
    #[serde(rename = "@blendMode")]
    pub blend_mode: u32,
    #[serde(rename = "@show")]
    pub show: u32,
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    #[serde(rename = "LayoutObject")]
    pub layout_object: LayoutObject,
}

impl ComposerMapOverview {
    pub fn new(frame_map_uuid: uuid::Uuid) -> Self {
        Self {
            position: 3,
            inverted: 0,
            name: "Overview 1".to_string(),
            centered: 0,
            uuid: QgisUuid::default(),
            frame_map: QgisUuid(frame_map_uuid),
            blend_mode: 0,
            show: 1,
            symbol: Symbol::fill_symbol(Color {
                red: 255,
                blue: 0,
                alpha: 75,
                green: 0,
            }),
            layout_object: LayoutObject::default_with_custom_properties_and_extra_option(),
        }
    }
}
