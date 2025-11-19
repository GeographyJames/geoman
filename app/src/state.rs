use ogcapi_types::common::LandingPage;
use utoipa::{OpenApi, openapi};

use crate::utoipa::ApiDoc;

pub struct AppState {
    pub landing_page: LandingPage,
    pub openapi: openapi::OpenApi,
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

impl AppState {
    pub fn new() -> Self {
        let landing_page = LandingPage {
            title: Some("GeoMan OGC API".to_string()),
            description: Some("Geospatial Features API".to_string()),
            links: vec![],
            ..Default::default()
        };
        let openapi = ApiDoc::openapi();

        Self {
            landing_page,
            openapi,
        }
    }
}
