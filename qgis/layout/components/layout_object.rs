use serde::Serialize;

use crate::data_defined_properties::DataDefinedProperties;

#[derive(Serialize, Default)]
pub struct LayoutObject {
    #[serde(rename = "dataDefinedProperties")]
    pub data_defined_properties: DataDefinedProperties,
    #[serde(rename = "customproperties", skip_serializing_if = "Option::is_none")]
    pub custom_properties: Option<CustomProperties>,
}

#[derive(Serialize, Default)]
pub struct CustomProperties {
    #[serde(rename = "Option")]
    pub option: Option<String>,
}

impl LayoutObject {
    pub fn default_with_custom_properties_and_extra_option() -> Self {
        let data_defined_properties = DataDefinedProperties::new_with_extra_option();
        LayoutObject {
            data_defined_properties,
            custom_properties: Some(CustomProperties::default()),
        }
    }
}
