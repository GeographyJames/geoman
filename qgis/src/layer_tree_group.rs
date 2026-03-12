use serde::Serialize;

use crate::qgis::{
    data_defined_properties::{PropertiesOption, PropertyOption},
    layer::{DataProvider, DataSource},
};

#[derive(Serialize, Default)]
pub struct LayerTreeGroup {
    customproperties: CustomProperties,
    #[serde(rename = "layer-tree-layer")]
    layer_tree_layers: Vec<LayerTreeLayer>,
    #[serde(rename = "custom-order")]
    custom_order: CustomOrder,
}

impl LayerTreeGroup {
    pub fn new() -> Self {
        Self {
            customproperties: CustomProperties::default(),
            layer_tree_layers: Vec::new(),
            custom_order: CustomOrder::default(),
        }
    }

    pub fn add_layer(
        &mut self,
        layer_id: String,
        layer_name: String,
        datasource: DataSource,
        legend_text: Option<String>,
        provider_key: DataProvider,
    ) {
        let mut layer_tree_layer = LayerTreeLayer {
            provider_key,
            legend_split_behavior: "0".into(),
            id: layer_id.clone(),
            patch_size: "-1,-1".into(),
            name: layer_name,
            source: datasource,
            checked: "Qt::Checked".into(),
            expanded: "1".into(),
            legend_exp: String::new(),
            customproperties: CustomProperties::default(),
        };
        if let Some(text) = legend_text {
            layer_tree_layer.customproperties.option = Some(PropertiesOption {
                option_type: "Map".to_string(),
                options: vec![
                    PropertyOption {
                        value: Some(text.clone()),
                        prop_type: Some("QString".into()),
                        name: "cached_name".into(),
                    },
                    PropertyOption {
                        value: Some(text),
                        prop_type: Some("QString".into()),
                        name: "legend/title-label".into(),
                    },
                ],
            })
        }

        self.layer_tree_layers.push(layer_tree_layer);
        self.custom_order
            .items
            .push(CustomOrderItem { content: layer_id });
    }
}

#[derive(Serialize)]
struct CustomOrder {
    #[serde(rename = "@enabled")]
    enabled: String,
    #[serde(rename = "item")]
    items: Vec<CustomOrderItem>,
}

impl Default for CustomOrder {
    fn default() -> Self {
        Self {
            enabled: "0".into(),
            items: Vec::new(),
        }
    }
}

#[derive(Serialize)]
struct CustomOrderItem {
    #[serde(rename = "$text")]
    content: String,
}

#[derive(Serialize, Default)]
struct CustomProperties {
    #[serde(rename = "Option")]
    option: Option<PropertiesOption>,
}

#[derive(Serialize)]
struct LayerTreeLayer {
    #[serde(rename = "@providerKey")]
    provider_key: DataProvider,
    #[serde(rename = "@legend_split_behavior")]
    legend_split_behavior: String,
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "@patch_size")]
    patch_size: String,
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@source")]
    source: DataSource,
    #[serde(rename = "@checked")]
    checked: String,
    #[serde(rename = "@expanded")]
    expanded: String,
    #[serde(rename = "@legend_exp")]
    legend_exp: String,
    customproperties: CustomProperties,
}
