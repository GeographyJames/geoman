use std::fmt::Display;

use serde::{Deserialize, Serialize};
use sqlx::prelude::Type;

#[derive(Serialize, Deserialize, Type)]
#[sqlx(transparent)]
pub struct ProjectName(pub String);

impl Display for ProjectName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
