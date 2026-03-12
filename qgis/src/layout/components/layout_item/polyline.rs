use crate::qgis::enums::LineCapStyle;
use crate::qgis::layout::components::LayoutObject;
use crate::qgis::layout::components::layout_item::Length;
use crate::qgis::{
    Symbol,
    enums::{LineJoinStyle, Units},
    layout::{
        Size,
        components::{Color, LayoutItem, Node, Nodes, Position},
    },
};

impl LayoutItem {
    pub fn polyline(nodes: Nodes, size: Size) -> Self {
        Self {
            arrowHeadFillColor: Some(Color::black().to_string()),
            arrowHeadOutlineColor: Some(Color::black().to_string()),
            arrowHeadWidth: Some(4.),
            markerMode: Some(0),
            outlineWidth: Some(1.),
            outline_width_m: Length {
                distance: 0.3,
                units: Units::Millimeter,
            },
            startMarkerMode: Some(0),
            opacity: 1.,
            item_type: 65645,
            visibility: 1,
            z_value: 1,
            nodes: Some(nodes),
            symbol: Some(Symbol::line_symbol(
                LineJoinStyle::Bevel,
                LineCapStyle::default(),
            )),
            frame_color: Color {
                red: 0,
                blue: 0,
                green: 0,
                ..Default::default()
            },

            size,
            layout_object: LayoutObject::default_with_custom_properties_and_extra_option(),
            ..Default::default()
        }
    }

    pub fn single_line(start: Position, end: Position) -> Self {
        let start_node = Node { x: 0., y: 0. };
        let end_node = Node {
            x: (end.x - start.x),
            y: (end.y - start.y),
        };
        let size = Size {
            width_mm: (end.x - start.x).abs(),
            height_mm: (end.y - start.y).abs(),
        };
        let mut polyline = Self::polyline(
            Nodes {
                nodes: vec![start_node, end_node],
            },
            size,
        );
        polyline.set_position(start);
        polyline
    }
}

#[cfg(test)]
mod tests {
    use crate::qgis::{
        layout::components::{LayoutItem, Position},
        tests::test_helpers::{SkipNode, xml_comparison},
    };

    #[test]
    fn single_line_works() {
        let start = Position { x: 5., y: 5. };
        let end = Position { x: 105., y: 5. };
        let layout_item = LayoutItem::single_line(start, end);
        let xml =
            quick_xml::se::to_string(&layout_item).expect("failed to serialize polyline to xml");
        let expected = include_str!("../examples/polyline.xml");
        let skip_config = vec![
            SkipNode {
                node_name: "LayoutItem".into(),
                attributes_to_skip: vec!["endMarkerFile".into(), "startMarkerFile".into()],
                children_to_skip: vec![],
                skip_text: false,
            },
            SkipNode {
                node_name: "LayoutObject".into(),
                attributes_to_skip: vec![],
                children_to_skip: vec!["customproperties".into()],
                skip_text: false,
            },
        ];

        let custom_properties =
            quick_xml::se::to_string(&layout_item.layout_object.data_defined_properties).unwrap();
        println!("{custom_properties}");
        xml_comparison(&xml, expected, Some(&skip_config));
    }
}
