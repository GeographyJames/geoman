use crate::common::{constants::REQUEST_FAILED, services::HttpClient};
use reqwest::{Response, header::AUTHORIZATION};

pub struct HttpService {
    pub endpoint: String,
}

impl HttpService {
    pub async fn get(&self, client: &HttpClient, auth_token: Option<&str>) -> Response {
        let mut req = client.get(&self.endpoint);
        if let Some(token) = auth_token {
            req = req.header(AUTHORIZATION, token)
        };
        req.send().await.expect(REQUEST_FAILED)
    }
}
