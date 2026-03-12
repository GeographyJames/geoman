use std::path::Path;

use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct QgisServerSettings {
    pub url: String,
    pub figure_config: QgisFigureConfig,
}

#[derive(Deserialize, Clone, Debug)]
pub struct QgisFigureConfig {
    pub figure_assets_directory: String,
    pub logo_filename: String,
    pub logo_height_pixels: u32,
    pub logo_width_pixels: u32,
    pub north_arrow_filename: String,
    pub north_arrow_height_pixels: u32,
    pub north_arrow_width_pixels: u32,
}

impl QgisFigureConfig {
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
