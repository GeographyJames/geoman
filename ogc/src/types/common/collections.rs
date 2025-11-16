use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::types::common::{Link, Links, link_relations::SELF, media_types::MediaType};

use super::Collection;

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
