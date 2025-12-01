use serde::{Deserialize, Serialize};

use crate::SubdivisionId;

#[derive(Serialize, Deserialize, Debug, sqlx::Type)]
#[sqlx(type_name = "app.subdivision")]
pub struct Subdivision {
    pub id: SubdivisionId,
    pub country_code: String,
    pub subdivision_code: String,
    pub name: String,
}
