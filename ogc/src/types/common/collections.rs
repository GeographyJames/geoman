use ogcapi_types::common::{Collection, Links};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct Collections {
    pub links: Links,
    pub collections: Vec<Collection>,
}
