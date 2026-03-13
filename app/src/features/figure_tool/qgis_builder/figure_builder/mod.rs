use uuid::Uuid;

use crate::{config::QgisFigureConfig, features::figure_tool::dtos::FigureOutputDTO};
use qgis::{
    enums::EPSGID,
    layout::{
        QgisLayoutBuilder, QgisLayoutMapBuilder, Size,
        components::{ComposerMapGrid, LayoutItem, Position},
    },
    layer::MapLayer,
};

use super::PrintResolution;

mod copyright_text;
mod id;
mod legend;
mod north_arrow;
mod scalebar;
mod text_box;

pub const FIG_TITLE_HEIGHT: f32 = 13.;
pub const FIG_SUBTITLE_HEIGHT: f32 = 12.;
pub const TEXT_BOX_HEIGHT: f32 = 10.;

pub struct FigureBuilder<'a> {
    pub layout_map_builders: Vec<QgisLayoutMapBuilder>,
    pub main_layout_map_uuid: Uuid,
    pub overview_map_uuid: Uuid,

    pub fig: &'a FigureOutputDTO,
    pub print_resolution: &'a PrintResolution,
    pub print_right: f32,
    pub print_bottom: f32,
    pub print_top: f32,
    pub print_left: f32,

    pub legend_width: f32,
    pub overview_map_top: f32,
    pub layout_items: Vec<LayoutItem>,
    pub margin: f32,
    pub print_width: f32,
    pub map_width: f32,
    pub config: Option<&'a QgisFigureConfig>,
    pub layers: &'a Vec<MapLayer>,
    pub print_height: f32,
    pub legend_left: f32,
}

impl<'a> FigureBuilder<'a> {
    pub fn new(
        fig: &'a FigureOutputDTO,
        print_resolution: &'a PrintResolution,
        config: Option<&'a QgisFigureConfig>,
        layers: &'a Vec<MapLayer>,
    ) -> FigureBuilder<'a> {
        let print_right = fig.page_width_mm - fig.margin_mm;
        let print_bottom = fig.page_height_mm - fig.margin_mm;

        let overview_map_top = print_bottom as f32
            - 2. * TEXT_BOX_HEIGHT
            - FIG_SUBTITLE_HEIGHT
            - FIG_TITLE_HEIGHT
            - fig
                .overview_map_base_map
                .as_ref()
                .map_or(0, |_| fig.legend_width_mm) as f32;

        Self {
            layout_map_builders: Vec::new(),
            main_layout_map_uuid: Uuid::new_v4(),
            print_resolution,
            fig,
            print_left: fig.margin_mm as f32,
            print_top: fig.margin_mm as f32,
            print_bottom: print_bottom as f32,
            print_right: print_right as f32,

            legend_width: fig.legend_width_mm as f32,
            overview_map_top,
            layout_items: Vec::new(),
            margin: fig.margin_mm as f32,
            print_width: (fig.page_width_mm - 2 * fig.legend_width_mm) as f32,
            config,
            map_width: (fig.map_right() - fig.margin_mm as u32) as f32,
            layers,
            print_height: (print_bottom - fig.margin_mm) as f32,
            overview_map_uuid: Uuid::new_v4(),
            legend_left: (fig.page_width_mm - fig.margin_mm - fig.legend_width_mm) as f32,
        }
    }

    pub fn build(mut self, include_ids: bool) -> Result<QgisLayoutBuilder, anyhow::Error> {
        if self.fig.legend_width_mm > 0 && self.fig.overview_map_base_map.is_some() {
            let overview_map = QgisLayoutMapBuilder {
                size: Size {
                    width_mm: self.legend_width as f64,
                    height_mm: self.legend_width as f64,
                },
                position: Position {
                    x: self.legend_left as f64,
                    y: self.overview_map_top as f64,
                },
                extent: self.fig.overview_map_extent,
                srs: Some(EPSGID::BNG),
                uuid: self.overview_map_uuid,
                id: "Overview Map".into(),
                map_grid: None,
                keep_layer_set: false,
                map_to_overview: self
                    .fig
                    .properties
                    .overview_frame
                    .unwrap_or(true)
                    .then_some(self.main_layout_map_uuid),
                z_value: Some(5),
            };
            self.layout_map_builders.push(overview_map);
            self.add_overview_map_copyright_text();
        }
        let main_map = QgisLayoutMapBuilder {
            size: Size {
                width_mm: self.map_width as f64,
                height_mm: self.print_height as f64,
            },

            position: Position {
                x: self.print_left as f64,
                y: self.print_top as f64,
            },
            extent: self.fig.map_extent,
            srs: Some(EPSGID::BNG),
            uuid: self.main_layout_map_uuid,

            map_grid: self.fig.properties.map_ticks.unwrap_or(true).then_some(
                ComposerMapGrid::new(self.fig.properties.grid_spacing.unwrap_or(1000)),
            ),
            id: "Main Map".into(),
            keep_layer_set: false,
            map_to_overview: None,
            z_value: None,
        };
        self.layout_map_builders.push(main_map);

        if include_ids {
            self.add_ids();
        };

        if self.fig.properties.north_arrow.unwrap_or(true) {
            self.add_north_arrow();
        }

        self.add_copyright_text();
        if self.fig.properties.scalebar.unwrap_or(true) {
            self.add_scalebar();
        }
        if self.legend_width > 0. {
            self.add_legend();
        }
        Ok(QgisLayoutBuilder {
            page_size: self.fig.page_size(),
            layout_maps: self.layout_map_builders,
            layout_items: self.layout_items,
            print_resolution: *self.print_resolution as u32,
            name: self.fig.layout_name(),
        })
    }
}
