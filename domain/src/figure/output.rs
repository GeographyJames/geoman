use std::{collections::HashSet, fmt::Display};

use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::{
    domain::{
        dtos::{BaseMapOutputDTO, FigureLayerOutputDTO, FigureProperties, Id, Point, UserId},
        enums::{FigureLayerDatasourceOutput, FigureStatus, ProjectLayer},
    },
    qgis::figure::PrintResolution,
};

use crate::qgis::{Extent, layout::Size};

pub const OVERVIEW_MAP_SCALE: u32 = 1_000_000;

#[derive(Debug)]
pub struct QgisProjectName(pub String);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FigureOutputDTO {
    pub id: Id,
    pub main_map_base_map_id: Option<Id>,
    pub overview_map_base_map_id: Option<Id>,
    pub project_id: Id,
    pub project_name: String,
    pub qgis_project_uuid: uuid::Uuid,
    pub added_by: UserId,
    pub added_by_first_name: String,
    pub added_by_last_name: String,
    pub last_updated_by: UserId,
    pub last_updated_by_first_name: String,
    pub last_updated_by_last_name: String,
    pub status: FigureStatus,
    pub added: chrono::DateTime<Utc>,
    pub last_updated: chrono::DateTime<Utc>,
    pub page_width_mm: i32,
    pub page_height_mm: i32,
    pub margin_mm: i32,
    pub legend_width_mm: i32,
    pub scale: i32,
    pub srid: i32,
    pub properties: FigureProperties,
    pub layers: Option<Vec<FigureLayerOutputDTO>>,
    #[serde(skip_serializing)]
    pub main_map_base_map: Option<BaseMapOutputDTO>,
    #[serde(skip_serializing)]
    pub overview_map_base_map: Option<BaseMapOutputDTO>,
    pub map_extent: Extent,
    pub target_coord: Point,
    pub overview_map_extent: Extent,
    pub target_layer_extent: Option<Extent>,
}

impl Display for FigureOutputDTO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<FigureOutputDTO> id: {}", self.id.0)
    }
}

impl FigureOutputDTO {
    pub fn map_right(&self) -> u32 {
        if self.properties.legend_height_percent.unwrap_or(100) < 100 {
            (self.page_width_mm - self.margin_mm) as u32
        } else {
            (self.page_width_mm - self.margin_mm - self.legend_width_mm) as u32
        }
    }

    pub fn page_size(&self) -> Size {
        Size {
            width_mm: self.page_width_mm as f64,
            height_mm: self.page_height_mm as f64,
        }
    }

    pub fn layer_names(&self) -> Vec<String> {
        let mut layer_names = Vec::new();
        if let Some(ref layers) = self.layers {
            for layer in layers {
                layer_names.push(layer.name.clone());
            }
        };
        layer_names
    }

    pub fn map_layer_names(&self) -> Vec<String> {
        let mut names: Vec<String> = Vec::new();
        if let Some(ref layers) = self.layers {
            for layer in layers {
                match layer.source {
                    FigureLayerDatasourceOutput::ProjectLayer(ProjectLayer::Invalid(_)) => {}
                    FigureLayerDatasourceOutput::SiteBoundary(_)
                    | FigureLayerDatasourceOutput::TurbineLayout(_)
                    | FigureLayerDatasourceOutput::ProjectLayer(ProjectLayer::Valid(_)) => {
                        if layer.properties.include_on_map {
                            names.push(layer.name.clone())
                        }
                    }
                }
            }
        }

        names
    }

    pub fn layout_name(&self) -> String {
        let mut layout_name = String::from(self.properties.title.as_deref().unwrap_or("untitled"));
        if let Some(subtitle) = &self.properties.subtitle {
            layout_name.push_str(&format!("-{}", subtitle));
        }
        slug::slugify(layout_name)
    }
    pub fn qgis_project_name(&self, print_resolution: &PrintResolution) -> QgisProjectName {
        match print_resolution {
            PrintResolution::High => QgisProjectName(self.qgis_project_uuid.to_string()),
            PrintResolution::Low => QgisProjectName(format!("{}_low-res", self.qgis_project_uuid)),
        }
    }
    pub fn filename_without_id(&self, suffix: &str) -> String {
        format!("{}.{}", self.filename(), suffix)
    }

    fn filename(&self) -> String {
        let mut filename = String::from(self.properties.title.as_deref().unwrap_or("untitled"));

        if let Some(figure_number) = self.properties.figure_number.as_deref() {
            filename.push('_');
            filename.push_str(figure_number);
        }
        if let Some(subtitle) = self.properties.subtitle.as_deref() {
            filename.push('_');
            filename.push_str(subtitle);
        }
        slug::slugify(filename)
    }

    pub fn filename_with_id(&self, suffix: &str) -> String {
        format!("{}_{:05}.{}", self.filename(), self.id.as_ref(), suffix)
    }

    pub fn user_id_with_initials_and_last_updated(&self) -> String {
        let mut user_id = format!("U{:04}", self.last_updated_by.as_ref());
        if let (Some(first_initial), Some(second_initial)) = (
            self.last_updated_by_first_name.chars().next(),
            self.last_updated_by_last_name.chars().next(),
        ) {
            user_id.push(first_initial);
            user_id.push(second_initial)
        };
        user_id.push(' ');
        user_id.push_str(&format!(
            "{}",
            self.last_updated.format("%d%m%y %H:%M:%S%Z")
        ));
        user_id
    }
    pub fn unique_boundary_ids_on_map(&self) -> HashSet<Id> {
        if let Some(ref layers) = self.layers {
            layers
                .iter()
                .filter(|l| l.properties.include_on_map)
                .filter_map(|l| {
                    if let FigureLayerDatasourceOutput::SiteBoundary(ref ds) = l.source {
                        Some(ds.id)
                    } else {
                        None
                    }
                })
                .collect()
        } else {
            HashSet::new()
        }
    }

    pub fn unique_layout_ids_on_map(&self) -> HashSet<Id> {
        if let Some(ref layers) = self.layers {
            layers
                .iter()
                .filter(|l| l.properties.include_on_map)
                .filter_map(|l| {
                    if let FigureLayerDatasourceOutput::TurbineLayout(ref ds) = l.source {
                        Some(ds.id)
                    } else {
                        None
                    }
                })
                .collect()
        } else {
            HashSet::new()
        }
    }
    pub fn set_basemap_urls_to_alt_urls(&mut self) {
        if let Some(ref mut map) = self.main_map_base_map {
            map.set_url_to_alt_url();
        }
        if let Some(ref mut map) = self.overview_map_base_map {
            map.set_url_to_alt_url();
        }
    }
}
