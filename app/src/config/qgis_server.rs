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
