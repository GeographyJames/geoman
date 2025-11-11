use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::ogc::types::common::Links;

/// A single collection in the OGC API
#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct Collection {
    /// Unique identifier for the collection (slug)
    pub id: String,

    /// Human-readable title
    pub title: String,

    /// Description of the collection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Links related to this collection
    pub links: Links,
}
