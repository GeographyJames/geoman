use ogcapi_types::common::{Crs, Extent, SpatialExtent};
use serde::Deserialize;

use crate::{CreateLinks, IntoOGCCollection, SupportedCrs, enums::CollectionId};

#[derive(Deserialize)]
pub struct ProjectCollection {
    pub id: CollectionId,
    pub title: String,
    pub description: Option<String>,
    pub storage_crs: Option<Crs>,
    pub extent: Option<SpatialExtent>,
    pub supported_crs: SupportedCrs,
}

impl IntoOGCCollection for ProjectCollection {
    fn into_ogc_collection(self, collections_url: &str) -> ogcapi_types::common::Collection {
        let Self {
            title,
            supported_crs,
            description,
            storage_crs,
            id,
            extent,
        } = self;
        let links = ogcapi_types::common::Collection::create_links(collections_url, &id);

        ogcapi_types::common::Collection {
            id: id.to_string(),
            title: Some(title),
            description,
            crs: supported_crs.into_inner(),
            links,
            storage_crs,

            extent: extent.map(|spatial| Extent {
                spatial: Some(spatial),
                temporal: None,
            }),

            ..Default::default()
        }
    }
}
