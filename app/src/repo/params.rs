pub mod project {
    #[derive(Default)]
    pub struct SelectAllParams {
        pub limit: Option<usize>,
    }
}

pub mod project_features {
    use domain::{ProjectCollectionId, ProjectId};

    use crate::types::ValidCrs;

    #[derive(Clone)]
    pub struct SelectAllParams {
        pub limit: Option<usize>,
        pub collection_id: ProjectCollectionId,
        pub project_id: Option<ProjectId>,
        pub crs: ValidCrs,
        pub bbox: Option<ogcapi_types::common::Bbox>,
        pub bbox_crs: Option<ValidCrs>,
    }

    #[derive(Clone)]
    pub struct SelectOneParams<'a> {
        pub project_id: Option<ProjectId>,
        pub crs: &'a ValidCrs,
    }
}

pub mod collections {
    use domain::ProjectId;

    pub struct SelectOneParams {
        pub project_id: ProjectId,
    }

    pub struct SelectAllParams {
        pub project_id: ProjectId,
    }
}
