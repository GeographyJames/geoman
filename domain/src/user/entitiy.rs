use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::{Team, UserId};

#[derive(Serialize, Deserialize, Debug, sqlx::Type, FromRow)]
#[sqlx(type_name = "app.user")]
pub struct User {
    pub id: UserId,
    pub first_name: String,
    pub last_name: String,
    pub clerk_id: Option<String>,
    pub team: Option<Team>,
}
