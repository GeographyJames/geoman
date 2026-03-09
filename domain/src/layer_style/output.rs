use serde::{Deserialize, Serialize};

use crate::domain::dtos::Id;

#[derive(Serialize, Deserialize)]
pub struct LayerStyleOutputDTO {
    pub id: Id,
    pub schema_name: Option<String>,
    pub table_name: Option<String>,
    pub stylename: String,
    pub description: Option<String>,
    pub geometry_type: Option<String>,
    pub use_as_default: Option<bool>,
    pub owner: String,
}
