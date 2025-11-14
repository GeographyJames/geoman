use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::types::common::{
    CollectionRow, Link, Links, link_relations::SELF, media_types::MediaType,
};

use super::Collection;

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct Collections {
    pub links: Links,
    pub collections: Vec<Collection>,
}

impl Collections {
    pub fn from_collection_rows(
        collection_rows: Vec<CollectionRow>,
        collections_url: &str,
    ) -> Self {
        let collections: Vec<Collection> = collection_rows
            .into_iter()
            .map(|row| Collection::from_collection_row(row, collections_url.to_string()))
            .collect();
        Self {
            links: vec![Link::new(&collections_url, SELF).mediatype(MediaType::Json)],
            collections,
        }
    }
    pub fn add_collection(mut self, collection: Collection) -> Self {
        self.collections.push(collection);
        self
    }
}
