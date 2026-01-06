use serde::{Deserialize, Serialize};

use crate::TeamId;

#[derive(Serialize, Deserialize, Debug, sqlx::Type, Default)]
#[sqlx(type_name = "app.team")]
pub struct Team {
    id: TeamId,
    name: String,
}
