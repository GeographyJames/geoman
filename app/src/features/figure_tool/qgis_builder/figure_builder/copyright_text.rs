use qgis::{
    enums::{HorizontalAlignment, ReferencePoint, VerticalAlignment},
    layout::{
        Size,
        components::{Color, LayoutItem, Position, TextBuffer},
    },
};

use super::FigureBuilder;

impl FigureBuilder<'_> {
    pub fn add_copyright_text(&mut self) {
        if let Some(ref copyright_text) = self.fig.properties.copyright_text {
            let text = match copyright_text {
                crate::features::figure_tool::CopyrightText::Default => {
                    if let Some(ref main_map) = self.fig.main_map_base_map {
                        main_map.data_provider.copyright_text.clone()
                    } else {
                        None
                    }
                }
                crate::features::figure_tool::CopyrightText::Custom => self
                    .fig
                    .properties
                    .custom_copyright_text
                    .as_ref()
                    .map(|s| s.to_owned()),

                crate::features::figure_tool::CopyrightText::None => None,
            };
            if let Some(text) = text {
                let mut copyright_text = LayoutItem::text(text, 10, None, Color::black());
                copyright_text.set_position(Position {
                    x: (self.legend_left) as f64,
                    y: (self.print_bottom) as f64,
                });
                copyright_text.size = Size {
                    height_mm: 100.,
                    width_mm: 100.,
                };
                copyright_text.z_value = 25;
                copyright_text.reference_point = ReferencePoint::BottomRight;
                copyright_text.valign = Some(VerticalAlignment::Bottom);
                copyright_text.halign = Some(HorizontalAlignment::Right);
                copyright_text.marginX = Some(4.);
                copyright_text.marginY = Some(4.);

                let mut text_buffer = TextBuffer::default_enabled();
                text_buffer.buffer_opacity = 0.7;
                text_buffer.buffer_size = 2.;

                if let Some(ref mut text_style) = copyright_text.text_style {
                    text_style.text_buffer = text_buffer
                }
                self.layout_items.push(copyright_text)
            }
        }
    }

    pub fn add_overview_map_copyright_text(&mut self) {
        if let Some(ref map) = self.fig.overview_map_base_map
            && let Some(ref text) = map.data_provider.copyright_text
        {
            let mut cp_text = LayoutItem::text(text.clone(), 7, None, Color::black());
            cp_text.set_position(Position {
                x: self.print_right as f64,
                y: (self.overview_map_top + self.legend_width) as f64,
            });
            cp_text.size = Size {
                width_mm: self.legend_width as f64,
                height_mm: self.legend_width as f64,
            };
            cp_text.z_value = 25;
            cp_text.reference_point = ReferencePoint::BottomRight;
            cp_text.marginX = Some(1.);
            cp_text.marginY = Some(1.);
            cp_text.halign = Some(HorizontalAlignment::Right);
            cp_text.valign = Some(VerticalAlignment::Bottom);
            let mut text_buffer = TextBuffer::default_enabled();
            text_buffer.buffer_opacity = 0.7;
            text_buffer.buffer_size = 1.;

            if let Some(ref mut text_style) = cp_text.text_style {
                text_style.text_buffer = text_buffer
            }

            self.layout_items.push(cp_text)
        }
    }
}
