use ogcapi_types::common::Crs;
use serde::{Deserialize, Serialize};
use serde_with::DisplayFromStr;
use utoipa::ToSchema;

use crate::types::common::{
    Link, Links,
    link_relations::{ITEMS, SELF},
    media_types::MediaType,
};

/// A single collection in the OGC API
#[serde_with::serde_as]
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

    /// The list of coordinate reference systems supported by the API; the first item is the default coordinate reference system.
    #[serde(default)]
    #[serde_as(as = "Vec<DisplayFromStr>")]
    #[schema(value_type = Vec<String>)]
    pub crs: Vec<Crs>,
}

impl Collection {
    pub fn new(
        id: String,
        title: String,
        description: Option<String>,
        collections_url: &str,
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
            crs: vec![Crs::default()],
        }
    }
}
