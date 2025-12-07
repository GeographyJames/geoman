use domain::{TeamId, UserId};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum AuthenticatedUser {
    AuthenticationId(String),
    User(UserContext),
}

#[derive(Clone, FromRow, Default, Serialize, Deserialize, Debug)]
pub struct UserContext {
    pub id: UserId,
    pub team_id: Option<TeamId>,
    pub admin: bool,
}
