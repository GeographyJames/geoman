use serde::{Deserialize, Serialize};

use crate::domain::enums::Status;

#[derive(Serialize, Deserialize)]
pub struct Project {
    id: u32,
    properties: ProjectProperties,
}

#[derive(Serialize, Deserialize)]
pub struct ProjectProperties {
    name: String,
    status: Status,
}
