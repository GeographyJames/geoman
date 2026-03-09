use crate::{
    enums::{HorizontalAlignment, NamedTextStyle, ReferencePoint, VerticalAlignment},
    figure::figure_builder::{
        FIG_SUBTITLE_HEIGHT, FIG_TITLE_HEIGHT, FigureBuilder, TEXT_BOX_HEIGHT,
    },
    layout::{
        PageSize, Size,
        components::{Color, LayoutItem, LegendTextStyles, Position, SizeInteger},
    },
};

impl FigureBuilder<'_> {
    pub fn add_legend(&mut self) {
        let logo_height = if let Some(cofig) = self.config {
            cofig.calculate_logo_height_mm(self.fig.legend_width_mm as f64)
        } else {
            0.
        };
        let logo_top = self.overview_map_top as f64
            - if self.fig.properties.logo.unwrap_or(true) {
                logo_height
            } else {
                0.
            };
        let legend_height = self.fig.properties.legend_height_percent.unwrap_or(100) as f32 / 100.
            * self.print_height;
        let legend_right = self.print_right as f64;
        let legend_left = (self.print_right - self.fig.legend_width_mm as f32) as f64;
        let legend_bottom = self.print_bottom as f64;

        let legend_top = legend_bottom - legend_height as f64;
        let legend_hcentre = legend_left + (self.legend_width / 2.) as f64;
        let mut items = Vec::new();

        let top_line = LayoutItem::single_line(
            Position {
                x: legend_left,
                y: legend_top,
            },
            Position {
                x: legend_right,
                y: legend_top,
            },
        );
        items.push(top_line);

        let right_line = LayoutItem::single_line(
            Position {
                x: legend_right,
                y: legend_top,
            },
            Position {
                x: legend_right,
                y: legend_bottom,
            },
        );
        items.push(right_line);
        let left_line = LayoutItem::single_line(
            Position {
                x: legend_left,
                y: legend_top,
            },
            Position {
                x: legend_left,
                y: legend_bottom,
            },
        );
        items.push(left_line);

        let bottom_line = LayoutItem::single_line(
            Position {
                x: legend_left,
                y: legend_bottom,
            },
            Position {
                x: legend_right,
                y: legend_bottom,
            },
        );
        items.push(bottom_line);

        items.push(LayoutItem::single_line(
            Position {
                x: legend_left,
                y: legend_bottom - TEXT_BOX_HEIGHT as f64,
            },
            Position {
                x: legend_right,
                y: legend_bottom - TEXT_BOX_HEIGHT as f64,
            },
        ));
        items.push(LayoutItem::single_line(
            Position {
                x: legend_left,
                y: legend_bottom - 2. * TEXT_BOX_HEIGHT as f64,
            },
            Position {
                x: legend_right,
                y: legend_bottom - 2. * TEXT_BOX_HEIGHT as f64,
            },
        ));
        items.push(LayoutItem::single_line(
            Position {
                x: legend_hcentre,
                y: legend_bottom - 2. * TEXT_BOX_HEIGHT as f64,
            },
            Position {
                x: legend_hcentre,
                y: legend_bottom,
            },
        ));

        self.add_text_box(
            Position {
                x: legend_left,
                y: legend_bottom - 2. * TEXT_BOX_HEIGHT as f64,
            },
            "LAST REVISION DATE:".into(),
            chrono::Local::now().format("%d/%m/%Y").to_string(),
        );

        self.add_text_box(
            Position {
                x: legend_hcentre,
                y: legend_bottom - 2. * TEXT_BOX_HEIGHT as f64,
            },
            "DOCUMENT STATUS:".into(),
            self.fig
                .properties
                .status
                .as_ref()
                .unwrap_or(&"".to_string())
                .to_owned(),
        );

        self.add_text_box(
            Position {
                x: legend_left,
                y: legend_bottom - TEXT_BOX_HEIGHT as f64,
            },
            format!(
                "SCALE @ {}",
                PageSize::from_dimensions(SizeInteger {
                    width_mm: self.fig.page_width_mm as u32,
                    height_mm: self.fig.page_height_mm as u32
                })
                .map(|p| p.name())
                .unwrap_or(format!(
                    "paper dimensions w: {}mm, h: {}mm",
                    self.fig.page_width_mm, self.fig.page_height_mm
                ))
            ),
            format!("1:{}", utils::format_with_commas(self.fig.scale as u32)),
        );

        if let Some(ref fig_no) = self.fig.properties.figure_number {
            let mut item = LayoutItem::text(
                fig_no.to_owned(),
                10,
                Some(NamedTextStyle::Bold),
                Color::black(),
            );
            item.set_position(Position {
                x: legend_hcentre,
                y: legend_bottom - TEXT_BOX_HEIGHT as f64,
            });
            item.halign = Some(HorizontalAlignment::Center);
            item.valign = Some(VerticalAlignment::Middle);
            item.size = Size {
                width_mm: self.fig.legend_width_mm as f64 / 2.,
                height_mm: TEXT_BOX_HEIGHT as f64,
            };
            items.push(item);
        }

        if let Some(ref title) = self.fig.properties.title {
            let mut title = LayoutItem::text(
                title.clone(),
                12,
                Some(NamedTextStyle::Bold),
                Color::black(),
            );
            title.set_position(Position {
                x: legend_left,
                y: legend_bottom - 2. * TEXT_BOX_HEIGHT as f64 - FIG_SUBTITLE_HEIGHT as f64,
            });
            title.size = Size {
                width_mm: self.fig.legend_width_mm as f64,
                height_mm: FIG_TITLE_HEIGHT as f64,
            };

            title.marginY = Some(1.);
            title.marginX = Some(2.);
            title.reference_point = ReferencePoint::BottomLeft;
            title.valign = Some(VerticalAlignment::Bottom);
            title.halign = Some(HorizontalAlignment::Center);
            items.push(title);
        }
        if let Some(ref subtitle) = self.fig.properties.subtitle {
            let mut subtitle = LayoutItem::text(
                subtitle.clone(),
                10,
                Some(NamedTextStyle::MediumItalic),
                Color::black(),
            );
            subtitle.set_position(Position {
                x: legend_left,
                y: legend_bottom - (2. * TEXT_BOX_HEIGHT as f64),
            });
            subtitle.size = Size {
                width_mm: self.fig.legend_width_mm as f64,
                height_mm: FIG_SUBTITLE_HEIGHT as f64,
            };
            subtitle.marginY = Some(1.);
            subtitle.marginX = Some(2.);
            subtitle.reference_point = ReferencePoint::BottomLeft;
            subtitle.valign = Some(VerticalAlignment::Top);
            subtitle.halign = Some(HorizontalAlignment::Center);
            items.push(subtitle)
        }

        if self.fig.properties.legend.unwrap_or(true) {
            let mut legend_content = LayoutItem::legend(
                self.main_layout_map_uuid,
                Size {
                    width_mm: self.fig.legend_width_mm as f64,
                    height_mm: logo_top - legend_top,
                },
                Position {
                    x: legend_left,
                    y: legend_top,
                },
                false,
                Some(self.layers),
                Some(LegendTextStyles {
                    title_size: 10,
                    title_style: Some(NamedTextStyle::Bold),
                    group_size: 10,
                    group_style: None,
                    subgroup_size: 10,
                    subgroup_style: None,
                    label_size: 10,
                    label_style: None,
                }),
            );
            legend_content.wrap_char = Some(r"\n".into());
            items.push(legend_content);
        }

        if self.fig.properties.logo.unwrap_or(true)
            && let Some(fig_config) = self.config
        {
            let size = Size {
                width_mm: self.fig.legend_width_mm as f64,
                height_mm: logo_height,
            };
            let mut logo = LayoutItem::svg_image(
                fig_config.logo_filepath(),
                size,
                Position {
                    x: legend_left,
                    y: self.overview_map_top as f64,
                },
                "Logo".into(),
            );
            logo.reference_point = ReferencePoint::BottomLeft;
            logo.anchor_point = Some(ReferencePoint::BottomCenter);
            items.push(logo);
        }

        if let Some(extra_legend_text) = self.fig.properties.extra_legend_text.as_ref() {
            let mut item = LayoutItem::text(extra_legend_text.clone(), 10, None, Color::black());
            item.set_position(Position {
                x: legend_left,
                y: legend_top,
            });
            item.size = Size {
                width_mm: self.fig.legend_width_mm as f64,
                height_mm: logo_top - legend_top,
            };

            item.marginX = Some(2.);
            item.marginY = Some((self.fig.properties.legend_text_vmargin.unwrap_or(0) + 2) as f32);
            let enable_html = self.fig.properties.enable_html.unwrap_or(false);
            if !enable_html {
                item.valign = Some(VerticalAlignment::Bottom);
            }
            item.htmlState = Some(enable_html as u8);
            items.push(item);
        }
        for item in items.iter_mut() {
            item.set_z_value(10);
        }
        let background = LayoutItem::rectangle(
            Size {
                width_mm: self.legend_width as f64,
                height_mm: legend_height as f64,
            },
            Position {
                x: legend_left,
                y: legend_top,
            },
        );
        self.layout_items.push(background);
        self.layout_items.extend(items);
    }
}
