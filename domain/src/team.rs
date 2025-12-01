use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::TeamId;

#[derive(Serialize, Deserialize, Debug, sqlx::Type)]
#[sqlx(type_name = "app.team_type")]
pub struct Team {
    id: TeamId,
    name: String,
}
