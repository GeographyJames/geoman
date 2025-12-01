use serde::{Deserialize, Serialize};

use crate::{Team, UserId};

#[derive(Serialize, Deserialize)]
pub struct User {
    id: UserId,
    clerk_id: Option<String>,
    team: Team,
}
