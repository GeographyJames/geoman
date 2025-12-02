use serde::{Deserialize, Serialize};

use crate::{Team, UserId};

#[derive(Serialize, Deserialize, Debug, sqlx::Type)]
#[sqlx(type_name = "app.user")]
pub struct User {
    id: UserId,
    first_name: String,
    last_name: String,
    clerk_id: Option<String>,
    team: Team,
}
