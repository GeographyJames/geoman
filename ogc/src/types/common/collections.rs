use ogcapi_types::common::{Collection, Links};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct Collections {
    pub links: Links,
    pub collections: Vec<Collection>,
}
