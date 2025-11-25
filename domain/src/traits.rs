use ogcapi_types::common::{
    Link,
    link_rel::{ITEMS, SELF},
    media_type::{GEO_JSON, JSON},
};

use crate::enums::CollectionId;

pub trait IntoOGCFeature {
    fn into_ogc_feature(self, collection_url: String) -> ogc::Feature
    where
        Self: Sized;
}

pub trait IntoOGCCollection {
    fn into_ogc_collection(self, collection_url: &str) -> ogcapi_types::common::Collection;
}

pub trait CreateLinks {
    fn create_links(collections_url: &str, id: &CollectionId) -> Vec<Link>;
}

impl CreateLinks for ogcapi_types::common::Collection {
    fn create_links(collections_url: &str, id: &CollectionId) -> Vec<Link> {
        vec![
            Link::new(format!("{}/{}", collections_url, id), SELF).mediatype(JSON),
            Link::new(format!("{}/{}/items", collections_url, id), ITEMS)
                .mediatype(GEO_JSON)
                .title("Items"),
        ]
    }
}
