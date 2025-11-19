use ogcapi_types::common::{Crs, Link, link_rel::SELF, media_type::JSON};

use crate::Collection;

pub struct Collections {
    pub collections: Vec<Collection>,
}

impl From<Vec<Collection>> for Collections {
    fn from(collections: Vec<Collection>) -> Self {
        Self { collections }
    }
}

impl Collections {
    pub fn into_ogc_collections(
        self,
        collections_url: &str,
        crs: Vec<Crs>,
    ) -> ogcapi_types::common::Collections {
        let collections = self
            .collections
            .into_iter()
            .map(|c| c.into_ogc_collection(collections_url, crs.clone()))
            .collect();
        let mut ogc_collections = ogcapi_types::common::Collections::new(collections);
        ogc_collections.crs = crs;
        ogc_collections.links = vec![Link::new(collections_url, SELF).mediatype(JSON)];
        ogc_collections
    }
}
