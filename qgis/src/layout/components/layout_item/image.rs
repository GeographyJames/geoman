use crate::{
    QgisUuid,
    data_defined_properties::DataDefinedProperties,
    enums::{LineJoinStyle, ReferencePoint},
    layout::components::{Color, LayoutItem, LayoutObject, Length, Position, Size},
};

impl LayoutItem {
    pub fn svg_image(file_path: String, size: Size, position: Position, id: String) -> Self {
        let mut item = LayoutItem {
            // Set image-specific attributes based on the svg_image.xml example
            anchor_point: Some(ReferencePoint::MiddleCenter),
            svg_fill_color: Some("255,255,255,255".to_string()),
            north_offset: Some(0.0),
            picture_rotation: Some(0.0),
            background: false,
            position: Position { x: 380.0, y: 242.0 },
            reference_point: crate::enums::ReferencePoint::MiddleCenter,
            mode: Some(0), // SVG mode
            position_lock: false,
            blend_mode: 0,
            opacity: 1.0,
            exclude_from_exports: 0,
            north_mode: Some(0),
            svg_border_width: Some(0.2),
            item_type: 65640, // Image type
            file: Some(file_path.to_string()),
            group_uuid: String::new(),
            frame: false,
            z_value: 3,
            uuid: QgisUuid(uuid::Uuid::new_v4()),
            size,
            item_rotation: 0,
            template_uuid: String::new(),
            visibility: 1,
            svg_border_color: Some("0,0,0,255".to_string()),
            frame_join_style: LineJoinStyle::Miter,
            map_uuid: None,
            outline_width_m: Length {
                distance: 0.3,
                units: crate::enums::Units::Millimeter,
            },
            resize_mode: Some(0),
            id,
            picture_width: None,
            picture_height: None,

            // Set frame and background colors
            frame_color: Color::black(),
            background_color: Color::white(),

            // Set layout object
            layout_object: LayoutObject {
                data_defined_properties: DataDefinedProperties::new_with_extra_option(),
                custom_properties: Some(Default::default()),
            },

            ..Default::default()
        };
        item.set_position(position);
        item
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        layout::{
            Size,
            components::{LayoutItem, Position},
        },
        tests::test_helpers::{SkipNode, xml_comparison},
    };

    #[test]
    fn svg_image_works() {
        let file_path = "".to_string();
        let image = LayoutItem::svg_image(
            file_path,
            Size {
                width_mm: 65.,
                height_mm: 65.,
            },
            Position { x: 380.0, y: 242.0 },
            "".into(),
        );
        let xml = quick_xml::se::to_string(&image).expect("failed to convert image to xml");
        let expected = include_str!("../examples/svg_image.xml");
        let skip_config = vec![SkipNode {
            node_name: "LayoutItem".into(),
            attributes_to_skip: vec![
                "mapUuid".into(),
                "file".into(),
                "pictureWidth".into(),
                "pictureHeight".into(),
                "anchorPoint".into(),
            ],
            children_to_skip: vec![],
            skip_text: false,
        }];
        xml_comparison(&xml, expected, Some(&skip_config));
    }
}
