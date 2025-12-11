use domain::{TeamId, UserId};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Clone, FromRow, Default, Serialize, Deserialize, Debug)]
pub struct AuthenticatedUser {
    pub id: UserId,
    pub first_name: String,
    pub last_name: String,
    pub username: Option<String>,
    pub team_id: TeamId,
    pub admin: bool,
}
