use crate::ogc::types::common::{ConformanceDeclaration, LandingPage, conformance_classes};

pub struct AppState {
    pub landing_page: LandingPage,
    pub conformance_declaration: ConformanceDeclaration,
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

        let mut conformance_declaration = ConformanceDeclaration::default();
        conformance_declaration.extend(&[
            conformance_classes::CORE,
            conformance_classes::GEOJSON,
            conformance_classes::OAS30,
        ]);

        Self {
            conformance_declaration,
            landing_page,
        }
    }
}
