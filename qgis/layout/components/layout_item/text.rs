use crate::{
    enums::{HorizontalAlignment, NamedTextStyle, Units, VerticalAlignment},
    layout::components::{Color, LayoutItem, LayoutObject, Length, TextStyle},
};

impl LayoutItem {
    pub fn text(
        text: String,
        font_size: u16,
        named_style: Option<NamedTextStyle>,
        colour: Color,
    ) -> Self {
        Self {
            labelText: Some(text),
            halign: Some(HorizontalAlignment::Left),
            valign: Some(VerticalAlignment::Top),
            htmlState: Some(0),
            marginX: Some(0.),
            marginY: Some(2.),
            item_type: 65641,
            z_value: 1,
            opacity: 1.,
            outline_width_m: Length {
                distance: 0.3,
                units: Units::Millimeter,
            },

            text_style: Some(TextStyle::new(font_size, named_style, colour)),
            visibility: 1,
            frame_color: Color {
                red: 0,
                blue: 0,
                green: 0,
                ..Default::default()
            },
            layout_object: LayoutObject::default_with_custom_properties_and_extra_option(),

            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        layout::{Size, components::Position},
        tests::test_helpers::{SkipNode, xml_comparison},
    };

    use super::*;

    #[test]
    fn text_item_works() {
        let mut layout_item = LayoutItem::text("Lorem ipsum".into(), 10, None, Color::black());
        layout_item.marginY = Some(0.);
        layout_item.set_position(Position { x: 5., y: 5. });
        layout_item.size = Size {
            width_mm: 100.,
            height_mm: 100.,
        };
        let skip_config = vec![
            SkipNode {
                node_name: "LayoutObject".into(),
                attributes_to_skip: vec![],
                children_to_skip: vec!["customproperties".into()],
                skip_text: false,
            },
            SkipNode {
                node_name: "Option".into(),
                attributes_to_skip: vec![],
                children_to_skip: vec!["Option".into()],
                skip_text: false,
            },
        ];
        let xml = quick_xml::se::to_string(&layout_item).expect("failed to generate xml string");
        let expected = include_str!("../examples/lorem-ipsum.xml");
        xml_comparison(&xml, expected, Some(&skip_config));
    }
}
