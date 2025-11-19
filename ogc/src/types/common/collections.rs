use ogcapi_types::common::{Collection, Link, Links, link_rel::SELF};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::types::common::media_types::MediaType;

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct Collections {
    pub links: Links,
    pub collections: Vec<Collection>,
}

impl Collections {
    pub fn new(collections_url: &str) -> Self {
        Collections {
            links: vec![Link::new(collections_url, SELF).mediatype(MediaType::Json)],
            collections: Vec::new(),
        }
    }
    pub fn add_collection(mut self, collection: Collection) -> Self {
        self.collections.push(collection);
        self
    }
    pub fn append_collections(mut self, mut collections: Vec<Collection>) -> Self {
        self.collections.append(&mut collections);
        self
    }
}
