use crate::{
    QgisUuid, Symbol,
    data_defined_properties::PropertyOption,
    enums::{LineCapStyle, LineJoinStyle, ReferencePoint, ScalebarUnits, Units},
    layout::{
        Size,
        components::{
            Color, LayoutItem, LayoutObject, Length, Position, TextStyle,
            layout_item::{NumericFormat, components::LayoutItemSymbol},
        },
    },
};

impl LayoutItem {
    pub fn scalebar(
        map_uuid: uuid::Uuid,
        size: Size,
        position: Position,
        units_per_segment: u32,
        scalebar_units: &ScalebarUnits,
    ) -> Self {
        let mut scalebar = Self::default();
        scalebar.set_position(position);

        // Basic layout item properties
        scalebar.item_type = 65646;
        scalebar.opacity = 1.0;
        scalebar.visibility = 1;
        scalebar.z_value = 18;

        scalebar.size = size;
        scalebar.outline_width_m = Length {
            distance: 0.3,
            units: Units::Millimeter,
        };
        scalebar.frame = false;
        scalebar.background = false;
        scalebar.position_lock = false;
        scalebar.item_rotation = 0;
        scalebar.blend_mode = 0;
        scalebar.exclude_from_exports = 0;
        scalebar.outlineWidth = Some(0.3);
        scalebar.reference_point = ReferencePoint::BottomLeft;

        // Scalebar-specific properties from XML
        scalebar.line_cap_style = Some("square".to_string());
        scalebar.subdivisions_height = Some(1.5);
        scalebar.min_bar_width = Some(50);
        scalebar.label_vertical_placement = Some(0);
        scalebar.max_bar_width = Some(150);
        scalebar.boxContentSpace = Some(1);
        scalebar.unit_label = Some(scalebar_units.to_string());
        scalebar.style = Some("Single Box".to_string());
        scalebar.num_segments = Some(2);
        scalebar.line_join_style = Some("miter".to_string());
        scalebar.num_units_per_segment = Some(units_per_segment);
        scalebar.num_map_units_per_scale_bar_unit = Some(1);
        scalebar.height = Some(3.0);
        scalebar.num_subdivisions = Some(1);
        scalebar.labelHorizontalPlacement = Some(0.0);
        scalebar.segment_millimeters = Some(20);
        scalebar.map_uuid = Some(QgisUuid(map_uuid));
        scalebar.segment_size_mode = Some(0);
        scalebar.label_bar_space = Some(3);
        scalebar.alignment = Some(0);
        scalebar.num_segments_left = Some(0);
        scalebar.unit_type = Some(scalebar_units.to_string());

        // Colors
        scalebar.frame_color = Color::black();
        scalebar.background_color = Color::white();
        scalebar.stroke_color = Some(Color::black());
        scalebar.fill_color = Some(Color::black());
        scalebar.fill_color2 = Some(Color::white());

        // Numeric format
        scalebar.numeric_format = Some(NumericFormat::default());

        // Text style for labels
        scalebar.text_style = Some(TextStyle::default());

        // Layout object
        scalebar.layout_object = LayoutObject {
            data_defined_properties: Default::default(),
            custom_properties: Some(Default::default()),
        };

        // Symbols - using existing line_symbol for now, could be customized later
        scalebar.line_symbol = Some(LayoutItemSymbol {
            symbol: Symbol::line_symbol(LineJoinStyle::Miter, LineCapStyle::default()),
        });
        scalebar.division_line_symbol = Some(LayoutItemSymbol {
            symbol: Symbol::line_symbol(LineJoinStyle::Miter, LineCapStyle::default()),
        });
        scalebar.subdivision_line_symbol = Some(LayoutItemSymbol {
            symbol: Symbol::line_symbol(LineJoinStyle::Miter, LineCapStyle::default()),
        });

        scalebar.fill_symbol1 = Some(LayoutItemSymbol {
            symbol: Symbol::fill_symbol(Color::black()),
        }); // Black fill symbol
        scalebar.fill_symbol2 = Some(LayoutItemSymbol {
            symbol: Symbol::fill_symbol(Color::white()),
        }); // White fill symbol
        scalebar
            .layout_object
            .data_defined_properties
            .option
            .options
            .push(PropertyOption {
                value: None,
                prop_type: None,
                name: "properties".into(),
            });

        scalebar
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::{
        enums::ScalebarUnits,
        layout::{
            Size,
            components::{LayoutItem, Position},
        },
        tests::test_helpers::{SkipNode, xml_comparison},
    };

    #[test]
    fn scalebar_item_works() {
        let size = Size {
            width_mm: 300.0,
            height_mm: 15.0,
        };
        let position = Position { x: 15.0, y: 270.0 };
        let layout_item = LayoutItem::scalebar(
            Uuid::new_v4(),
            size,
            position,
            1,
            &ScalebarUnits::Kilometers,
        );
        let skip_config = vec![
            SkipNode {
                node_name: "LayoutItem".into(),
                attributes_to_skip: vec!["mapUuid".into()],
                children_to_skip: vec![],
                skip_text: false,
            },
            SkipNode {
                node_name: "background".into(),
                attributes_to_skip: vec![],
                children_to_skip: vec!["symbol".into()],
                skip_text: false,
            },
        ];
        let xml = quick_xml::se::to_string(&layout_item)
            .expect("failed to convert scalbar to xml string");
        let expected = include_str!("../examples/scalebar.xml");
        xml_comparison(&xml, expected, Some(&skip_config));
    }
}
