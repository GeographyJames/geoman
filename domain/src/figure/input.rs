use std::path::Path;

use serde::Deserialize;

use crate::{
    DataProviderLayerId, ProjectId, UserId, enums::Status, figure::FigureProperties,
    figure_layer::FigureLayerInputDTO,
};

#[derive(Debug)]
pub struct FigureInputDTO {
    pub project_id: ProjectId,
    pub properties: FigureProperties,
    pub main_map_base_map_id: Option<DataProviderLayerId>,
    pub overvier_map_base_map_id: Option<DataProviderLayerId>,
    pub qgis_project_uuid: uuid::Uuid,
    pub user_id: UserId,
    pub status: Status,
    pub page_width_mm: u32,
    pub page_height_mm: u32,
    pub scale: u32,
    pub legend_width_mm: u32,
    pub margin_mm: u32,
    pub srid: u16,
    pub layers: Vec<FigureLayerInputDTO>,
    pub figure_config: Option<FigStaticConfig>,
}

#[derive(Debug)]
pub struct FigStaticConfig {
    pub figure_assets_directory: String,
    pub logo_filename: String,
    pub logo_height_pixels: u32,
    pub logo_width_pixels: u32,
    pub north_arrow_filename: String,
    pub north_arrow_height_pixels: u32,
    pub north_arrow_width_pixels: u32,
}

impl FigStaticConfig {
    pub fn calculate_logo_height_mm(&self, width_mm: f64) -> f64 {
        let aspect_ratio = self.logo_height_pixels as f64 / self.logo_width_pixels as f64;
        width_mm * aspect_ratio
    }
    pub fn logo_filepath(&self) -> String {
        Path::new(&self.figure_assets_directory)
            .join(Path::new(&self.logo_filename))
            .to_string_lossy()
            .to_string()
    }
    pub fn north_arrow_filepath(&self) -> String {
        Path::new(&self.figure_assets_directory)
            .join(Path::new(&self.north_arrow_filename))
            .to_string_lossy()
            .to_string()
    }
}
