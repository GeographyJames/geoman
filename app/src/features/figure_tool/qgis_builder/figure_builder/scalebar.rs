use crate::qgis::{
    enums::ScalebarUnits,
    layout::{
        Size,
        components::{LayoutItem, Position, TextBuffer},
    },
};

use super::FigureBuilder;

impl FigureBuilder<'_> {
    pub fn add_scalebar(&mut self) {
        let units_per_segment = self.fig.properties.scalebar_units_per_segment.unwrap_or(1);
        let units = self
            .fig
            .properties
            .scalebar_units
            .as_ref()
            .unwrap_or(&ScalebarUnits::Kilometers);
        let mut scalebar = LayoutItem::scalebar(
            self.main_layout_map_uuid,
            Size {
                width_mm: (self.map_width - 20.) as f64,
                height_mm: 11.,
            },
            Position {
                x: (self.print_left + 2.5) as f64,
                y: (self.print_bottom - 2.5) as f64,
            },
            units_per_segment,
            units,
        );
        if let Some(ref mut text_style) = scalebar.text_style {
            let mut text_buffer = TextBuffer::default_enabled();
            text_buffer.buffer_opacity = 0.7;
            text_style.text_buffer = text_buffer
        }
        self.layout_items.push(scalebar);
    }
}
