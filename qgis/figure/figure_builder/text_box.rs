use crate::qgis::{
    enums::{HorizontalAlignment, VerticalAlignment},
    figure::figure_builder::{FigureBuilder, TEXT_BOX_HEIGHT},
    layout::{
        Size,
        components::{Color, LayoutItem, Position},
    },
};

impl FigureBuilder<'_> {
    pub fn add_text_box(&mut self, position: Position, title: String, content: String) {
        let mut title_tb = LayoutItem::text(title, 6, None, Color::black());
        let size = Size {
            width_mm: self.legend_width as f64 / 2.,
            height_mm: TEXT_BOX_HEIGHT as f64,
        };
        title_tb.size = size;
        title_tb.set_position(position);
        title_tb.marginX = Some(0.5);
        title_tb.marginY = Some(0.5);
        title_tb.set_z_value(10);
        self.layout_items.push(title_tb);

        let mut content_tb = LayoutItem::text(content, 10, None, Color::black());
        content_tb.set_position(position);
        content_tb.size = size;
        content_tb.halign = Some(HorizontalAlignment::Center);
        content_tb.valign = Some(VerticalAlignment::Bottom);
        content_tb.marginY = Some(2.);
        content_tb.set_z_value(10);
        self.layout_items.push(content_tb)
    }
}
