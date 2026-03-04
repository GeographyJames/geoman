use domain::DataProviderId;
use domain::enums::DataProviderServiceType;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
pub struct DataProviderServiceInputPayload {
    pub provider_id: DataProviderId,
    pub name: String,
    pub service_type: DataProviderServiceType,
    pub base_url: String,
    pub description: Option<String>,
}
