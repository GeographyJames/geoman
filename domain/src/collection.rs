use ogc::MediaType;
use ogcapi_types::common::{
    Crs, Link,
    link_rel::{ITEMS, SELF},
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Collection {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
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
            ..
        } = self;
        let links = vec![
            Link::new(format!("{}/{}", collections_url, slug), SELF).mediatype(MediaType::Json),
            Link::new(format!("{}/{}/items", collections_url, slug), ITEMS)
                .mediatype(MediaType::GeoJson)
                .title("Items"),
        ];
        ogcapi_types::common::Collection {
            id: slug,
            title: Some(title),
            description: description,
            crs,
            links,
            ..Default::default()
        }
    }
}
