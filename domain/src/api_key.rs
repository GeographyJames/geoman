use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{KeyId, UserId};

#[derive(Serialize, Deserialize)]
pub struct ApiKey {
    pub id: KeyId,
    pub name: String,
    pub created: DateTime<Utc>,
    pub last_used: Option<DateTime<Utc>>,
    pub expiry: DateTime<Utc>,
}

pub struct ApiKeyInputDTO {
    pub user_id: UserId,
    pub name: String,
    pub key_hash: KeyHash,
}

pub struct KeyHash(pub String);
