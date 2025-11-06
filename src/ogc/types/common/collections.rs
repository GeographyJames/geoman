use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::ogc::types::common::Links;

use super::Collection;

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct Collections {
    pub links: Links,
    pub collections: Vec<Collection>,
}

impl Collections {
    pub fn new(collections: Vec<Collection>) -> Self {
        Collections {
            links: Vec::new(),
            collections,
        }
    }
}
