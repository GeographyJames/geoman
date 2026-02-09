use std::fmt::Display;

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
        id: impl Display,
    ) -> Response {
        auth_request(client.get(&format!("{}/{}", &self.endpoint, id)), auth)
            .send()
            .await
            .expect(REQUEST_FAILED)
    }
    pub async fn patch_json<B: Serialize>(
        &self,
        client: &HttpClient,
        id: impl Display,
        auth: Option<&Auth>,
        body: &B,
    ) -> Response {
        auth_request(
            client
                .patch(&format!("{}/{}", &self.endpoint, id))
                .json(body),
            auth,
        )
        .send()
        .await
        .expect(REQUEST_FAILED)
    }
    pub async fn post_form(
        &self,
        client: &HttpClient,
        form: reqwest::multipart::Form,
        id: impl Display,
        auth: Option<&Auth>,
    ) -> Response {
        auth_request(
            client
                .post(&format!("{}/{}", self.endpoint, id))
                .multipart(form),
            auth,
        )
        .send()
        .await
        .expect(REQUEST_FAILED)
    }
}
