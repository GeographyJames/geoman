use reqwest::RequestBuilder;

pub struct HttpClient {
    pub client: reqwest::Client,
    base_url: String,
}

impl HttpClient {
    pub fn new(base_url: String) -> Self {
        let client = reqwest::Client::new();
        Self { client, base_url }
    }

    pub fn get(&self, endpoint: impl AsRef<str>) -> RequestBuilder {
        self.client
            .get(format!("{}{}", self.base_url, endpoint.as_ref()))
    }
    pub fn post(&self, endpoint: impl AsRef<str>) -> RequestBuilder {
        self.client
            .post(format!("{}{}", self.base_url, endpoint.as_ref()))
    }

    pub fn patch(&self, endpoint: impl AsRef<str>) -> RequestBuilder {
        self.client
            .patch(format!("{}{}", self.base_url, endpoint.as_ref()))
    }
}
