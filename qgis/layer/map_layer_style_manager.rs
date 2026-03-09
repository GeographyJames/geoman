use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct MapLayerStyleManager {
    #[serde(rename = "@current")]
    current: String,
    #[serde(rename = "map-layer-style")]
    map_layer_tyle: MapLayerStyle,
}

impl Default for MapLayerStyleManager {
    fn default() -> Self {
        Self {
            current: "default".into(),
            map_layer_tyle: Default::default(),
        }
    }
}

#[derive(Serialize, Clone)]
struct MapLayerStyle {
    #[serde(rename = "@name")]
    name: String,
}

impl Default for MapLayerStyle {
    fn default() -> Self {
        Self {
            name: "default".into(),
        }
    }
}
