use serde::{Deserialize, Serialize};
use sqlx::prelude::Type;

#[derive(Serialize, Deserialize, Type)]
#[sqlx(transparent)]
pub struct ProjectName(pub String);
