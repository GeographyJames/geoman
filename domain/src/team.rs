use serde::{Deserialize, Serialize};

use crate::{BusinessUnitId, TeamId};

#[derive(Serialize, Deserialize, Debug, sqlx::Type, Default)]
#[sqlx(type_name = "app.team")]
pub struct Team {
    pub id: TeamId,
    pub name: String,
    pub business_unit_id: Option<BusinessUnitId>,
}
