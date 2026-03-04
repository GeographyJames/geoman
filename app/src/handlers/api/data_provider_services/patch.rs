use domain::enums::DataProviderServiceType;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
pub struct DataProviderServiceUpdatePayload {
    pub name: Option<String>,
    pub service_type: Option<DataProviderServiceType>,
    pub base_url: Option<String>,
    pub description: Option<String>,
}
