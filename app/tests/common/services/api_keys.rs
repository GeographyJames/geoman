use reqwest::{RequestBuilder, Response, header::AUTHORIZATION};

use crate::common::{constants::REQUEST_FAILED, services::HttpClient, types::SessionToken};
use app::handlers::api::keys::RequestPayload;

pub struct ApiKeysService {
    pub endpoint: String,
}

fn auth_request(req: RequestBuilder, token: Option<&SessionToken>) -> RequestBuilder {
    if let Some(token) = token {
        return req.header(AUTHORIZATION, &token.jwt);
    }
    req
}

impl ApiKeysService {
    pub async fn generate_api_key(
        &self,
        client: &HttpClient,
        token: Option<&SessionToken>,
    ) -> Response {
        let payload = RequestPayload {
            key_name: uuid::Uuid::new_v4().to_string(),
        };
        auth_request(client.post(&self.endpoint).json(&payload), token)
            .send()
            .await
            .expect(REQUEST_FAILED)
    }

    pub async fn get_all(&self, client: &HttpClient, token: Option<&SessionToken>) -> Response {
        auth_request(client.get(&self.endpoint), token)
            .send()
            .await
            .expect(REQUEST_FAILED)
    }
}
