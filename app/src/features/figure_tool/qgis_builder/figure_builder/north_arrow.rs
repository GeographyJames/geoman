use qgis::{
    enums::ReferencePoint,
    layout::{
        Size,
        components::{LayoutItem, Position},
    },
};

use super::FigureBuilder;

impl FigureBuilder<'_> {
    pub fn add_north_arrow(&mut self) {
        if let Some(config) = self.config {
            let mut north_arrow = LayoutItem::north_arrow(
                config.north_arrow_filepath(),
                Size {
                    width_mm: 10.,
                    height_mm: 20.,
                },
                Position {
                    x: (self.margin + 5.) as f64,
                    y: (self.margin + 5.) as f64,
                },
                self.main_layout_map_uuid,
            );
            north_arrow.anchor_point = Some(ReferencePoint::TopLeft);
            self.layout_items.push(north_arrow);
        };
    }
}
