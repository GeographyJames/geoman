use reqwest::Response;

use crate::common::{
    constants::{AUTHORISATION_HEADER, REQUEST_FAILED},
    services::HttpClient,
};

pub struct ApiKeysService {
    pub endpoint: String,
}

impl ApiKeysService {
    pub async fn generate_api_key(
        &self,
        client: &HttpClient,
        auth_token: Option<&str>,
    ) -> Response {
        let mut req = client.post(&self.endpoint);
        if let Some(token) = auth_token {
            req = req.header(AUTHORISATION_HEADER, token)
        };

        req.send().await.expect(REQUEST_FAILED)
    }
}
