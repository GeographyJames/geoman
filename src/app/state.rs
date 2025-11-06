use sqlx::PgPool;

use crate::{
    ogc::types::common::{ConformanceDeclaration, LandingPage, conformance_classes},
    repo::PostgresRepo,
};

pub struct AppState {
    pub landing_page: LandingPage,
    pub conformance_declaration: ConformanceDeclaration,
    pub repo: PostgresRepo,
}

impl AppState {
    pub fn new(db_pool: PgPool) -> Self {
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

        let repo = PostgresRepo { db_pool };

        Self {
            conformance_declaration,
            landing_page,
            repo,
        }
    }
}
