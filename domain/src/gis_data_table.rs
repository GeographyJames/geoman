use ogcapi_types::common::{Crs, Extent, SpatialExtent};

use crate::{CreateLinks, IntoOGCCollection, SupportedCrs, TableName, enums::GeometryType};

pub struct GisDataTable {
    pub table_name: TableName,
    pub schema_name: String,
    pub storage_crs: Option<Crs>,
    pub extent: Option<SpatialExtent>,
    pub description: Option<String>,
    pub owner: String,
    pub geometry_type: Option<GeometryType>,
    pub supported_crs: SupportedCrs,
}

impl IntoOGCCollection for GisDataTable {
    fn into_ogc_collection(self, collections_url: &str) -> ogcapi_types::common::Collection {
        let Self {
            table_name,
            storage_crs,
            extent,
            description,
            supported_crs,
            ..
        } = self;
        let id = crate::enums::CollectionId::DatabaseTable(table_name);
        let links = ogcapi_types::common::Collection::create_links(collections_url, &id);
        ogcapi_types::common::Collection {
            id: id.to_string(),
            title: Some(id.to_string()),
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
