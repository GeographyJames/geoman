use reqwest::Response;

use crate::common::{
    constants::{AUTHORISATION_HEADER, REQUEST_FAILED},
    services::{HttpClient, clerk::ClerkSessionToken},
};
use app::handlers::api::keys::RequestPayload;

pub struct ApiKeysService {
    pub endpoint: String,
}

impl ApiKeysService {
    pub async fn generate_api_key(
        &self,
        client: &HttpClient,
        session_token: Option<&ClerkSessionToken>,
    ) -> Response {
        let payload = RequestPayload {
            key_name: uuid::Uuid::new_v4().to_string(),
        };
        let mut req = client.post(&self.endpoint).json(&payload);
        if let Some(token) = session_token {
            req = req.header(AUTHORISATION_HEADER, &token.jwt)
        };

        req.send().await.expect(REQUEST_FAILED)
    }
}
