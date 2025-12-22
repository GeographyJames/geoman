use serde::{Deserialize, Serialize};

use crate::TechnologyId;

#[derive(Serialize, Deserialize)]
pub struct Technology {
    pub id: TechnologyId,
    pub name: String,
}
