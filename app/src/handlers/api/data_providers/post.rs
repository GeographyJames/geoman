use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
pub struct DataProviderInputPayload {
    pub name: String,
    pub description: Option<String>,
    pub country_code: Option<String>,
    pub subdivision: Option<String>,
}
