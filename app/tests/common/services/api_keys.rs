use domain::KeyId;
use reqwest::Response;

use crate::common::{Auth, constants::REQUEST_FAILED, helpers::auth_request, services::HttpClient};
use app::handlers::api::keys::ApiKeyReqPayload;

pub struct ApiKeysService {
    pub endpoint: String,
}

impl ApiKeysService {
    pub async fn generate_api_key(&self, client: &HttpClient, auth: Option<&Auth>) -> Response {
        let payload = ApiKeyReqPayload {
            key_name: uuid::Uuid::new_v4().to_string(),
        };
        auth_request(client.post(&self.endpoint).json(&payload), auth)
            .send()
            .await
            .expect(REQUEST_FAILED)
    }

    pub async fn get_all(&self, client: &HttpClient, auth: Option<&Auth>) -> Response {
        auth_request(client.get(&self.endpoint), auth)
            .send()
            .await
            .expect(REQUEST_FAILED)
    }

    pub async fn revoke(&self, client: &HttpClient, id: KeyId, auth: Option<&Auth>) -> Response {
        auth_request(
            client.patch(format!("{}/{}/revoke", self.endpoint, id.0)),
            auth,
        )
        .send()
        .await
        .expect(REQUEST_FAILED)
    }

    pub async fn renew(&self, client: &HttpClient, id: KeyId, auth: Option<&Auth>) -> Response {
        auth_request(
            client.patch(format!("{}/{}/renew", self.endpoint, id.0)),
            auth,
        )
        .send()
        .await
        .expect(REQUEST_FAILED)
    }
}
