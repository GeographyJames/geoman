use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::ogc::types::common::Link;

/// OGC API - Features Landing Page
///
/// The landing page provides links to the API definition, conformance
/// declaration, and data collections.
#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct LandingPage {
    /// Title of the API
    pub title: String,
    /// Description of the API
    pub description: String,
    /// Links to related resources
    pub links: Vec<Link>,
}
