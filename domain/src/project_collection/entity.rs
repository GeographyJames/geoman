use ogcapi_types::common::{Crs, Extent, SpatialExtent};
use serde::Deserialize;
use serde_json::Map;

use crate::{
    CreateLinks, IntoOGCCollection, ProjectId, SupportedCrs,
    enums::{CollectionId, GeometryType},
};

#[derive(Deserialize)]
pub struct ProjectCollection {
    pub id: CollectionId,
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub storage_crs: Option<Crs>,
    pub extent: Option<SpatialExtent>,
    pub supported_crs: SupportedCrs,
    pub geometry_type: GeometryType,
    pub project_id: Option<ProjectId>,
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
            geometry_type,
            slug,
            project_id,
        } = self;
        let links = ogcapi_types::common::Collection::create_links(collections_url, &id);
        let additional_properties = Map::from_iter([
            (
                "geometry_type".to_string(),
                serde_json::json!(geometry_type),
            ),
            ("slug".to_string(), serde_json::json!(slug)),
            (
                "project_id".to_string(),
                serde_json::json!(project_id.map(|p| p.0)),
            ),
        ]);

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
            additional_properties,
            ..Default::default()
        }
    }
}
