use serde::{Deserialize, Serialize};

use crate::enums::Status;

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

pub struct ProjectRow {
    pub id: i32,
    pub name: String,
    pub slug: String,
}
