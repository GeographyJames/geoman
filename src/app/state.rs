use crate::ogc::types::common::LandingPage;

pub struct AppState {
    pub landing_page: LandingPage,
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

impl AppState {
    pub fn new() -> Self {
        let landing_page = LandingPage {
            title: "GeoMan OGC API".to_string(),
            description: "Geospatial Features API".to_string(),
            links: vec![],
        };

        Self { landing_page }
    }
}
