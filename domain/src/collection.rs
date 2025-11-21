use ogcapi_types::common::{
    Crs, Extent, Link, SpatialExtent,
    link_rel::{ITEMS, SELF},
    media_type::{GEO_JSON, JSON},
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Collection {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub storage_crs_srid: Option<i32>,
    pub extent: Option<SpatialExtent>,
}

impl Collection {
    pub fn into_ogc_collection(
        self,
        collections_url: &str,
        crs: Vec<Crs>,
    ) -> ogcapi_types::common::Collection {
        let Self {
            title,

            description,
            storage_crs_srid,
            id,
            extent,
        } = self;
        let links = vec![
            Link::new(format!("{}/{}", collections_url, id), SELF).mediatype(JSON),
            Link::new(format!("{}/{}/items", collections_url, id), ITEMS)
                .mediatype(GEO_JSON)
                .title("Items"),
        ];

        ogcapi_types::common::Collection {
            id: id.to_string(),
            title: Some(title),
            description,
            crs,
            links,
            storage_crs: storage_crs_srid.map(|srid| Crs::from_srid(srid)),

            extent: extent.map(|spatial| Extent {
                spatial: Some(spatial),
                temporal: None,
            }),

            ..Default::default()
        }
    }
}
