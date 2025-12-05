use crate::common::{
    constants::REQUEST_FAILED,
    helpers::auth_request,
    services::{HttpClient, auth_service::SessionToken},
};
use reqwest::Response;
use serde::Serialize;

pub struct HttpService {
    pub endpoint: String,
}

impl HttpService {
    pub async fn get(&self, client: &HttpClient, token: Option<&SessionToken>) -> Response {
        auth_request(client.get(&self.endpoint), token)
            .send()
            .await
            .expect(REQUEST_FAILED)
    }
    pub async fn post_json<B: Serialize>(
        &self,
        client: &HttpClient,
        token: Option<&SessionToken>,
        body: &B,
    ) -> Response {
        auth_request(client.post(&self.endpoint).json(body), token)
            .send()
            .await
            .expect(REQUEST_FAILED)
    }
    pub async fn get_one(
        &self,
        client: &HttpClient,
        token: Option<&SessionToken>,
        id: impl AsRef<str>,
    ) -> Response {
        auth_request(
            client.get(&format!("{}/{}", &self.endpoint, id.as_ref())),
            token,
        )
        .send()
        .await
        .expect(REQUEST_FAILED)
    }
}
