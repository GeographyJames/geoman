use std::net::IpAddr;

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
    pub last_used_ip: Option<IpAddr>,
    pub last_used_user_agent: Option<String>,
}

pub struct ApiKeyInputDTO {
    pub name: String,
    pub key_hash: KeyHash,
}

pub struct KeyHash(pub String);
