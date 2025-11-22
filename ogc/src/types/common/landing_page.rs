use ogcapi_types::common::Links;
use serde::Serialize;
use utoipa::ToSchema;

/// OGC API - Features Landing Page
///
/// The landing page provides links to the API definition, conformance
/// declaration, and data collections.
#[derive(Serialize, ToSchema)]
pub struct LandingPage {
    /// Title of the API
    pub title: String,
    /// Description of the API
    pub description: String,
    /// Links to related resources
    pub links: Links,
}
