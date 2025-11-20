use ogcapi_types::common::{
    Crs, Link,
    link_rel::{ITEMS, SELF},
    media_type::{GEO_JSON, JSON},
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Collection {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    // pub storage_crs: Option<Crs>,
}

impl Collection {
    pub fn into_ogc_collection(
        self,
        collections_url: &str,
        crs: Vec<Crs>,
    ) -> ogcapi_types::common::Collection {
        let Self {
            title,
            slug,
            description,
            // storage_crs,
            id: _,
        } = self;
        let links = vec![
            Link::new(format!("{}/{}", collections_url, slug), SELF).mediatype(JSON),
            Link::new(format!("{}/{}/items", collections_url, slug), ITEMS)
                .mediatype(GEO_JSON)
                .title("Items"),
        ];
        ogcapi_types::common::Collection {
            id: slug,
            title: Some(title),
            description,
            crs,
            links,
            storage_crs: None,
            ..Default::default()
        }
    }
}
