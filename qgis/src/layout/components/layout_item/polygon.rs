use crate::layout::components::layout_item::Length;
use crate::layout::components::{LayoutObject, Position};
use crate::{
    Symbol,
    enums::Units,
    layout::{
        Size,
        components::{Color, LayoutItem, Node, Nodes},
    },
};

impl LayoutItem {
    pub fn polygon(nodes: Nodes, size: Size) -> Self {
        Self {
            outline_width_m: Length {
                distance: 0.3,
                units: Units::Millimeter,
            },
            opacity: 1.,
            item_type: 65644,
            visibility: 1,
            z_value: 2,
            nodes: Some(nodes),
            symbol: Some(Symbol::fill_symbol(Color::white())),
            frame_color: Color::black(),
            background_color: Color::white(),
            size,
            layout_object: LayoutObject::default_with_custom_properties_and_extra_option(),
            ..Default::default()
        }
    }
    pub fn rectangle(size: Size, position: Position) -> Self {
        let nodes = Nodes {
            nodes: vec![
                Node { x: 0., y: 0. },
                Node {
                    x: size.width_mm,
                    y: 0.,
                },
                Node {
                    x: size.width_mm,
                    y: size.height_mm,
                },
                Node {
                    x: 0.,
                    y: size.height_mm,
                },
            ],
        };
        let mut polygon = Self::polygon(nodes, size);
        polygon.set_position(position);
        polygon
    }
}
