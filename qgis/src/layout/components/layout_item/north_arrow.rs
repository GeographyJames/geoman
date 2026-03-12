use crate::{
    QgisUuid,
    enums::ReferencePoint,
    layout::{
        Size,
        components::{LayoutItem, Position},
    },
};

impl LayoutItem {
    pub fn north_arrow(
        file_path: String,
        size: Size,
        position: Position,
        map_uuid: uuid::Uuid,
    ) -> Self {
        let mut image = LayoutItem::svg_image(file_path, size, position, "North Arrow".into());
        image.map_uuid = Some(QgisUuid(map_uuid));
        image.id = "North Arrow".into();
        image.mode = Some(2);
        image.reference_point = ReferencePoint::TopLeft;
        image
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
    fn north_arrow_works() {
        let north_arrow = LayoutItem::north_arrow(
            "".into(),
            Size {
                width_mm: 54.485,
                height_mm: 130.87,
            },
            Position {
                x: 32.969,
                y: 58.438,
            },
            uuid::Uuid::new_v4(),
        );
        let xml = quick_xml::se::to_string(&north_arrow).expect("failed to convert image to xml");
        let expected = include_str!("../examples/north_arrow.xml");
        let skip_config = vec![SkipNode {
            node_name: "LayoutItem".into(),
            attributes_to_skip: vec![
                "file".into(),
                "pictureWidth".into(),
                "pictureHeight".into(),
                "anchorPoint".into(),
                "zValue".into(),
            ],
            children_to_skip: vec![],
            skip_text: false,
        }];
        xml_comparison(&xml, expected, Some(&skip_config));
    }
}
