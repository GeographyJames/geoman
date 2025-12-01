use serde::{Deserialize, Serialize};

use crate::TeamId;

#[derive(Serialize, Deserialize)]
pub struct Team {
    id: TeamId,
    name: String,
}
