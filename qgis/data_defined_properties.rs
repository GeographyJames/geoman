use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct DataDefinedProperties {
    #[serde(rename = "Option")]
    pub option: PropertiesOption,
}

impl DataDefinedProperties {
    pub fn new_with_extra_option() -> Self {
        Self {
            option: PropertiesOption::new_with_extra_option(),
        }
    }
}
#[derive(Serialize, Deserialize, Clone)]
pub struct PropertiesOption {
    #[serde(rename = "@type")]
    pub option_type: String,
    #[serde(rename = "Option")]
    pub options: Vec<PropertyOption>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PropertyOption {
    #[serde(rename = "@value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub prop_type: Option<String>,
    #[serde(rename = "@name")]
    pub name: String,
}

impl Default for PropertiesOption {
    fn default() -> Self {
        Self {
            option_type: "Map".to_string(),
            options: vec![
                PropertyOption {
                    value: Some("".to_string()),
                    prop_type: Some("QString".to_string()),
                    name: "name".to_string(),
                },
                PropertyOption {
                    value: Some("collection".to_string()),
                    prop_type: Some("QString".to_string()),
                    name: "type".to_string(),
                },
            ],
        }
    }
}

impl PropertiesOption {
    pub fn new_with_extra_option() -> Self {
        let mut item = Self::default();
        item.options.push(PropertyOption {
            value: None,
            prop_type: None,
            name: "properties".into(),
        });
        item
    }
}
