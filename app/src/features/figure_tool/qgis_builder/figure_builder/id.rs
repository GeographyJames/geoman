use crate::qgis::{
    enums::{HorizontalAlignment, ReferencePoint, VerticalAlignment},
    layout::{
        Size,
        components::{Color, LayoutItem, Position},
    },
};

use super::FigureBuilder;

impl FigureBuilder<'_> {
    pub fn add_ids(&mut self) {
        let mut figure_id = LayoutItem::text(
            format!(
                "{:06}:{}",
                self.fig.id.as_ref(),
                self.fig.qgis_project_name(self.print_resolution).0
            ),
            8,
            None,
            Color::grey(),
        );

        let text_size = Size {
            width_mm: self.print_width as f64,
            height_mm: self.margin as f64,
        };

        figure_id.set_position(Position {
            x: self.margin as f64,
            y: self.margin as f64,
        });
        figure_id.size = text_size;
        figure_id.valign = Some(VerticalAlignment::Bottom);
        figure_id.marginY = Some(0.);
        figure_id.reference_point = ReferencePoint::BottomLeft;
        self.layout_items.push(figure_id);

        let mut asset_ids = String::new();
        let project_id = format!(
            "P{:05}{}",
            self.fig.project_id.0,
            self.fig
                .project_name
                .chars()
                .take(3)
                .collect::<String>()
                .to_uppercase()
        );
        asset_ids.push_str(&project_id);
        let boundry_ids = self.fig.unique_boundary_ids_on_map();
        if !boundry_ids.is_empty() {
            let boundary_ids: Vec<String> = boundry_ids
                .iter()
                .map(|id| format!("SB{:05}", id.as_ref()))
                .collect();
            asset_ids.push_str(&format!(" {}", boundary_ids.join(" ")));
        }
        let layout_ids = self.fig.unique_layout_ids_on_map();
        if !layout_ids.is_empty() {
            let layout_ids: Vec<String> = layout_ids
                .iter()
                .map(|id| format!("TL{:05}", id.as_ref()))
                .collect();

            asset_ids.push_str(&format!(" {}", layout_ids.join(" ")));
        }
        asset_ids.push_str(&format!(
            " {}",
            self.fig.user_id_with_initials_and_last_updated()
        ));
        let mut asset_ids = LayoutItem::text(asset_ids, 8, None, Color::grey());
        asset_ids.set_position(Position {
            x: self.print_right as f64,
            y: self.margin as f64,
        });
        asset_ids.size = text_size;
        asset_ids.marginY = Some(0.);
        asset_ids.valign = Some(VerticalAlignment::Bottom);
        asset_ids.halign = Some(HorizontalAlignment::Right);
        asset_ids.reference_point = ReferencePoint::BottomRight;
        self.layout_items.push(asset_ids)
    }
}
