use serde::{Deserialize, Serialize};

use crate::TechnologyId;

#[derive(Serialize, Deserialize, Debug, sqlx::Type)]
#[sqlx(type_name = "app.technology")]
pub struct Technology {
    id: TechnologyId,
    name: String,
}
