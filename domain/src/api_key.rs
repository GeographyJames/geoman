use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::KeyId;

#[derive(Serialize, Deserialize)]
pub struct ApiKey {
    pub id: KeyId,
    pub name: String,
    pub created: DateTime<Utc>,
    pub last_used: Option<DateTime<Utc>>,
    pub expiry: DateTime<Utc>,
}
