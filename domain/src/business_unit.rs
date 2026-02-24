use serde::{Deserialize, Serialize};

use crate::BusinessUnitId;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct BusinessUnit {
    pub id: BusinessUnitId,
    pub name: String,
}
