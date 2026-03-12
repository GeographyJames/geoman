use serde::Serialize;

use crate::{
    LayerTreeGroup, QgisUuid,
    enums::{NamedTextStyle, ReferencePoint, Units},
    layer::MapLayer,
    layout::components::{Color, LayoutObject, Length, Position, Size, TextStyle},
};

use super::LayoutItem;

#[derive(Serialize)]
pub struct LegendStyles {
    style: Vec<LegendStyle>,
}

#[derive(Serialize)]
pub struct LegendStyle {
    #[serde(rename = "@alignment")]
    alignment: u32,
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@indent")]
    indent: u32,
    #[serde(rename = "@marginBottom", skip_serializing_if = "Option::is_none")]
    margin_bottom: Option<f32>,
    #[serde(rename = "@marginTop", skip_serializing_if = "Option::is_none")]
    margin_top: Option<f32>,
    #[serde(rename = "@marginLeft", skip_serializing_if = "Option::is_none")]
    margin_left: Option<f32>,
    #[serde(rename = "text-style")]
    text_style: TextStyle,
}

impl LegendStyles {
    fn create_text_style(
        font_size: u16,
        multiline_height: f32,
        style: Option<NamedTextStyle>,
    ) -> TextStyle {
        let mut text_style = TextStyle::default();
        text_style.multiline_height = multiline_height;
        text_style.font_size = font_size;
        text_style.named_style = style.map(|s| s.to_string()).unwrap_or("".into());
        // You would customize the text style here based on font_size
        // For now, using default but you can set font_size property if TextStyle supports it
        text_style
    }
}

impl LegendStyles {
    fn new(text_sizes: Option<LegendTextStyles>) -> Self {
        let text_styles = text_sizes.unwrap_or_default();
        Self {
            style: vec![
                // 1. Title style
                LegendStyle {
                    alignment: 1,
                    name: "title".to_string(),
                    indent: 0,
                    margin_bottom: Some(3.5),
                    margin_top: None,
                    margin_left: None,
                    text_style: LegendStyles::create_text_style(
                        text_styles.title_size,
                        1.1,
                        text_styles.title_style,
                    ),
                },
                // 2. Group style
                LegendStyle {
                    alignment: 1,
                    name: "group".to_string(),
                    indent: 0,
                    margin_bottom: None,
                    margin_top: Some(3.0),
                    margin_left: None,
                    text_style: LegendStyles::create_text_style(
                        text_styles.group_size,
                        1.1,
                        text_styles.group_style,
                    ),
                },
                // 3. Subgroup style
                LegendStyle {
                    alignment: 1,
                    name: "subgroup".to_string(),
                    indent: 0,
                    margin_bottom: None,
                    margin_top: Some(3.0),
                    margin_left: None,
                    text_style: LegendStyles::create_text_style(
                        text_styles.subgroup_size,
                        1.1,
                        text_styles.subgroup_style,
                    ),
                },
                // 4. Symbol style
                LegendStyle {
                    alignment: 1,
                    name: "symbol".to_string(),
                    indent: 0,
                    margin_bottom: None,
                    margin_top: Some(2.5),
                    margin_left: None,
                    text_style: LegendStyles::create_text_style(10, 1., None),
                },
                // 5. Symbol label style
                LegendStyle {
                    alignment: 1,
                    name: "symbolLabel".to_string(),
                    indent: 0,
                    margin_bottom: None,
                    margin_top: Some(2.0),
                    margin_left: Some(2.0),
                    text_style: LegendStyles::create_text_style(
                        text_styles.label_size,
                        1.1,
                        text_styles.label_style,
                    ),
                },
            ],
        }
    }
}

pub struct LegendTextStyles {
    pub title_size: u16,
    pub title_style: Option<NamedTextStyle>,
    pub group_size: u16,
    pub group_style: Option<NamedTextStyle>,
    pub subgroup_size: u16,
    pub subgroup_style: Option<NamedTextStyle>,
    pub label_size: u16,
    pub label_style: Option<NamedTextStyle>,
}

impl Default for LegendTextStyles {
    fn default() -> Self {
        Self {
            title_size: 16,
            title_style: None,
            group_size: 14,
            subgroup_style: None,
            subgroup_size: 12,
            group_style: None,
            label_size: 12,
            label_style: None,
        }
    }
}

impl LayoutItem {
    pub fn legend(
        map_uuid: uuid::Uuid,
        size: Size,
        position: Position,
        resize_to_contents: bool,
        layers: Option<&Vec<MapLayer>>,
        text_sizes: Option<LegendTextStyles>,
    ) -> Self {
        let layer_tree_group = if let Some(layers) = layers {
            let mut layer_tree = LayerTreeGroup::new();
            for layer in layers.iter().filter(|l| l.include_on_legend) {
                layer_tree.add_layer(
                    layer.id.clone(),
                    layer.layername.clone(),
                    layer.datasource.clone(),
                    layer.legend_text.clone(),
                    layer.provider.text.clone(),
                );
            }
            Some(layer_tree)
        } else {
            None
        };
        Self {
            // Basic layout item properties
            item_type: 65642, // Legend type
            opacity: 1.0,
            visibility: 1,
            z_value: 1,
            position_on_page: position,
            position,
            size,
            outline_width_m: Length {
                distance: 0.3,
                units: Units::Millimeter,
            },
            frame: false,
            background: false,
            position_lock: false,
            item_rotation: 0,
            blend_mode: 0,
            exclude_from_exports: 0,
            reference_point: ReferencePoint::TopLeft,

            // Legend-specific properties from XML
            title_alignment: Some(1),
            box_space: Some(2),
            equal_column_width: Some(0),
            max_symbol_size: Some(0),
            wms_legend_width: Some(50),
            resize_to_contents: Some(resize_to_contents as u8),
            wrap_char: Some(String::new()),
            symbol_width: Some(7),
            raster_border: Some(1),
            column_count: Some(1),
            symbol_height: Some(4),
            title: Some("Legend".to_string()),
            split_layer: Some(0),
            min_symbol_size: Some(0),
            raster_border_width: Some(0),
            raster_border_color: Some("0,0,0,255".to_string()),
            column_space: Some(2),
            symbol_alignment: Some(1),
            wms_legend_height: Some(25),
            legend_filter_by_atlas: Some(0),

            // Link to map
            legend_map_uuid: Some(QgisUuid(map_uuid)),

            // Colors
            frame_color: Color::black(),
            background_color: Default::default(),

            // Layout object
            layout_object: LayoutObject::default_with_custom_properties_and_extra_option(),

            // Styles
            styles: Some(LegendStyles::new(text_sizes)),

            // Layer Tree
            layer_tree_group,

            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        layer::{DataSource, PgSource, QgisMapLayerBuilder, WkbType},
        layout::{
            Size,
            components::{LayoutItem, Position},
        },
        srs::SpatialRefSys,
        tests::test_helpers::{SkipNode, generate_pg_datasource, xml_comparison},
    };

    #[test]
    fn legend_works() {
        let size = Size {
            width_mm: 70.,
            height_mm: 100.,
        };
        let position = Position { x: 345., y: 5. };
        let skip_config = vec![
            SkipNode {
                node_name: "LayoutObject".into(),
                children_to_skip: vec!["customproperties".into()],
                attributes_to_skip: vec![],
                skip_text: false,
            },
            SkipNode {
                node_name: "background".into(),
                skip_text: false,
                children_to_skip: vec!["symbol".into()],
                attributes_to_skip: vec![],
            },
        ];

        let legend = LayoutItem::legend(uuid::Uuid::new_v4(), size, position, true, None, None);
        let xml =
            quick_xml::se::to_string(&legend).expect("failed to convert legend to xml string");
        let expected = include_str!("../examples/basic_legend.xml");
        xml_comparison(&xml, expected, Some(&skip_config));
    }

    #[test]
    fn legend_with_custom_properties() {
        let size = Size {
            width_mm: 70.,
            height_mm: 100.,
        };
        let position = Position { x: 345., y: 5. };
        let skip_config = vec![
            SkipNode {
                node_name: "LayoutObject".into(),
                children_to_skip: vec!["customproperties".into()],
                attributes_to_skip: vec![],
                skip_text: false,
            },
            SkipNode {
                node_name: "background".into(),
                skip_text: false,
                children_to_skip: vec!["symbol".into()],
                attributes_to_skip: vec![],
            },
            SkipNode {
                node_name: "item".into(),
                skip_text: true,
                children_to_skip: vec![],
                attributes_to_skip: vec![],
            },
            SkipNode {
                node_name: "layer-tree-layer".into(),
                skip_text: false,
                children_to_skip: vec![],
                attributes_to_skip: vec!["id".into()],
            },
        ];

        let mut ds = generate_pg_datasource();
        ds.authcfg = None;
        ds.srid = None;
        ds.r#type = None;
        ds.source = PgSource::SQL("SELECT id, geom FROM app.site_boundaries WHERE id = 1".into());
        let layer = QgisMapLayerBuilder {
            layer_name: "main-boundary".into(),
            legend_text: Some("alternative legend text".into()),
            include_on_legend: true,
            datasource: DataSource::Postgres(ds),
            srs: Some(SpatialRefSys::wgs84()),
        }
        .build_vector(WkbType::MultiPolygon);

        let legend = LayoutItem::legend(
            uuid::Uuid::new_v4(),
            size,
            position,
            false,
            Some(&vec![layer]),
            None,
        );
        let xml =
            quick_xml::se::to_string(&legend).expect("failed to convert legend to xml string");
        let expected = include_str!("../examples/legend_with_custom_properties_and_text.xml");
        xml_comparison(&xml, expected, Some(&skip_config));
    }
}
