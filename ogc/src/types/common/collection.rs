use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::types::common::{
    Link, Links,
    link_relations::{ITEMS, SELF},
    media_types::MediaType,
};

pub struct CollectionRow {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
}

/// A single collection in the OGC API
#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
#[non_exhaustive]
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

impl Collection {
    pub fn new(
        id: String,
        title: String,
        description: Option<String>,
        collections_url: String,
    ) -> Self {
        let links = vec![
            Link::new(format!("{}/{}", collections_url, id), SELF).mediatype(MediaType::Json),
            Link::new(format!("{}/{}/items", collections_url, id), ITEMS)
                .mediatype(MediaType::GeoJson)
                .title("Items"),
        ];
        Self {
            id,
            title,
            description,
            links,
        }
    }
    pub fn from_collection_row(row: CollectionRow, collections_url: String) -> Self {
        Collection::new(row.slug, row.title, row.description, collections_url)
    }
}
