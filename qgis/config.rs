use serde::Deserialize;

/// QGIS figure generation configuration (logo path, etc.)
#[derive(Deserialize, Clone, Debug)]
pub struct QgisFigureConfig {
    pub logo_path: String,
    /// Aspect ratio (height / width) of the logo image
    pub logo_aspect_ratio: f64,
    pub north_arrow_path: String,
}

impl QgisFigureConfig {
    pub fn logo_filepath(&self) -> String {
        self.logo_path.clone()
    }

    pub fn north_arrow_filepath(&self) -> String {
        self.north_arrow_path.clone()
    }

    pub fn calculate_logo_height_mm(&self, legend_width_mm: f64) -> f64 {
        legend_width_mm * self.logo_aspect_ratio
    }
}
