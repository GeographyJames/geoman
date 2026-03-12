use serde::Serialize;

#[derive(Serialize, Default)]
pub struct ItemClippingSettings {
    #[serde(rename = "@enabled")]
    enabled: i16,
    #[serde(rename = "@clippingType")]
    clipping_type: i16,
}
