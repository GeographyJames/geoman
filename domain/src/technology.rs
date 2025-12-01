use serde::{Deserialize, Serialize};

use crate::TechnologyId;

#[derive(Serialize, Deserialize, Debug)]
pub struct Technology {
    id: TechnologyId,
    name: String,
}
