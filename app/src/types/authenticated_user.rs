use domain::{TeamId, UserId};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Clone, FromRow, Default, Serialize, Deserialize)]
pub struct AuthenticatedUser {
    pub id: UserId,
    pub team_id: Option<TeamId>,
    pub admin: bool,
}
