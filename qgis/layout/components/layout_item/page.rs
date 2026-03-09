use crate::{
    enums::Units,
    layout::{
        Size,
        components::{Color, LayoutItem, Length},
    },
};

impl LayoutItem {
    pub fn page(page_size: Size) -> Self {
        Self {
            item_type: 65638,
            size: page_size,
            opacity: 1.,
            visibility: 1,
            background: true,
            outline_width_m: Length {
                distance: 0.3,
                units: Units::Millimeter,
            },
            frame_color: Color::black(),
            symbol: Some(Default::default()),
            ..Default::default()
        }
    }
}
