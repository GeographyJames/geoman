use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ClerkWebhookPayload {
    #[serde(rename = "type")]
    pub event_type: String,
    pub data: Data,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Data {
    pub id: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}
