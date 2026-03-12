use serde::Serialize;

#[derive(Serialize)]
pub struct ClippingSettings {
    #[serde(rename = "@enabled")]
    pub enabled: u16,
    #[serde(rename = "@restrictLayers", skip_serializing_if = "Option::is_none")]
    pub restrict_layers: Option<u16>,
    #[serde(rename = "@clippingType")]
    pub clipping_type: u16,
    #[serde(rename = "@forceLabelsInside")]
    pub force_labels_inside: u16,

    #[serde(rename = "@clipSource", skip_serializing_if = "Option::is_none")]
    pub clip_source: Option<String>,
    #[serde(rename = "layersToClip", skip_serializing_if = "Option::is_none")]
    pub layers_to_clip: Option<String>,
}

impl Default for ClippingSettings {
    fn default() -> Self {
        Self {
            enabled: Default::default(),
            restrict_layers: Default::default(),
            clipping_type: 1,
            force_labels_inside: Default::default(),
            layers_to_clip: Default::default(),
            clip_source: Default::default(),
        }
    }
}

impl ClippingSettings {
    pub fn atlas() -> Self {
        Self {
            restrict_layers: Some(Default::default()),
            layers_to_clip: Some(Default::default()),
            ..Default::default()
        }
    }
}

impl ClippingSettings {
    pub fn item() -> Self {
        Self {
            clip_source: Some(Default::default()),
            ..Default::default()
        }
    }
}
