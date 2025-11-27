use reqwest::RequestBuilder;
use std::fmt::Display;

pub struct HttpClient {
    pub client: reqwest::Client,
    base_url: String,
}

impl HttpClient {
    pub fn new(base_url: String) -> Self {
        let client = reqwest::Client::new();
        Self { client, base_url }
    }

    pub fn get<T: Display>(&self, endpoint: T) -> RequestBuilder {
        self.client
            .get(format!("{}{}", self.base_url, endpoint.to_string()))
    }
    pub fn post<T: Display>(&self, endpoint: T) -> RequestBuilder {
        self.client.post(format!("{}{}", self.base_url, endpoint))
    }
}
