use ogcapi_types::common::{Crs, Link, link_rel::SELF, media_type::JSON};

use crate::{GisDataTable, IntoOGCCollection, ProjectCollection};

pub struct Collections {
    pub project_collections: Vec<ProjectCollection>,
    pub gis_data_tables: Vec<GisDataTable>,
}

impl Collections {
    pub fn into_ogc_collections(self, collections_url: &str) -> ogcapi_types::common::Collections {
        let collections: Vec<_> = self
            .project_collections
            .into_iter()
            .map(|c| c.into_ogc_collection(collections_url))
            .chain(
                self.gis_data_tables
                    .into_iter()
                    .map(|c| c.into_ogc_collection(collections_url)),
            )
            .collect();
        let mut ogc_collections = ogcapi_types::common::Collections::new(collections);
        ogc_collections.crs = vec![Crs::default()];

        ogc_collections.links = vec![Link::new(collections_url, SELF).mediatype(JSON)];
        ogc_collections
    }
}
