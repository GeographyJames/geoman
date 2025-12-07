use crate::common::{Auth, constants::REQUEST_FAILED, helpers::auth_request, services::HttpClient};
use reqwest::Response;
use serde::Serialize;

pub struct HttpService {
    pub endpoint: String,
}

impl HttpService {
    pub async fn get(&self, client: &HttpClient, auth: Option<&Auth>) -> Response {
        auth_request(client.get(&self.endpoint), auth)
            .send()
            .await
            .expect(REQUEST_FAILED)
    }
    pub async fn post_json<B: Serialize>(
        &self,
        client: &HttpClient,
        auth: Option<&Auth>,
        body: &B,
    ) -> Response {
        auth_request(client.post(&self.endpoint).json(body), auth)
            .send()
            .await
            .expect(REQUEST_FAILED)
    }
    pub async fn get_one(
        &self,
        client: &HttpClient,
        auth: Option<&Auth>,
        id: impl AsRef<str>,
    ) -> Response {
        auth_request(
            client.get(&format!("{}/{}", &self.endpoint, id.as_ref())),
            auth,
        )
        .send()
        .await
        .expect(REQUEST_FAILED)
    }
}
