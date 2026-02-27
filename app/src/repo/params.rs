pub mod project {

    use domain::enums::Status;
    use ogcapi_types::common::Crs;

    pub struct SelectOneParams<'a> {
        pub crs: &'a Crs,
    }

    pub struct SelectAllParams<'a> {
        pub limit: Option<usize>,
        pub crs: &'a Crs,
        pub _bbox: Option<&'a ogcapi_types::common::Bbox>,
        pub _bbox_crs: Option<&'a Crs>,
        pub status: Option<Vec<Status>>,
    }
}

pub mod project_features {
    use domain::{ProjectCollectionId, ProjectId, enums::Status};
    use ogcapi_types::common::Crs;

    #[derive(Clone)]
    pub struct SelectAllParams {
        pub limit: Option<usize>,
        pub collection_id: ProjectCollectionId,
        pub project_id: ProjectId,
        pub crs: Crs,
        pub bbox: Option<ogcapi_types::common::Bbox>,
        pub bbox_crs: Option<Crs>,
        pub offset: Option<usize>,
        pub status: Option<Vec<Status>>,
    }

    #[derive(Clone)]
    pub struct SelectOneParams<'a> {
        pub project_id: ProjectId,
        pub crs: &'a Crs,
    }
}

pub mod project_collections {
    use domain::{ProjectId, enums::Status};

    pub struct SelectOneParams {
        pub project_id: ProjectId,
        pub status: Option<Vec<Status>>,
    }

    pub struct SelectAllParams {
        pub project_id: ProjectId,
        pub status: Option<Vec<Status>>,
    }
}

pub mod features {
    use domain::TableName;
    use ogcapi_types::common::Crs;

    pub struct SelectOneParams<'a> {
        pub schema: &'a str,
        pub table: TableName,
        pub crs: Crs,
    }

    #[derive(Clone)]
    pub struct SelectAllParams {
        pub schema: &'static str,
        pub table: TableName,
        pub limit: Option<usize>,
        pub offset: Option<usize>,
        pub bbox: Option<ogcapi_types::common::Bbox>,
        pub bbox_crs: Option<Crs>,
        pub crs: Crs,
    }
}
pub mod turbine_layout_features {
    use domain::{ProjectId, enums::Status};
    use ogcapi_types::common::Crs;

    #[derive(Clone)]
    pub struct SelectAllParams {
        pub project_id: ProjectId,
        pub crs: Crs,
        pub limit: Option<usize>,
        pub offset: Option<usize>,
        pub status: Option<Vec<Status>>,
    }
}

pub mod api_keys {
    use domain::UserId;

    pub struct SelectAllParams {
        pub user_id: UserId,
    }
}

pub mod user_id {
    use std::net::IpAddr;

    pub struct SelectOneParams {
        pub ip_address: Option<IpAddr>,
        pub user_agent: Option<String>,
    }
}
