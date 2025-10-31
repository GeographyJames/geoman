use reqwest::RequestBuilder;

pub struct HttpClient {
    pub client: reqwest::Client,
    base_url: String,
}

impl HttpClient {
    pub fn new(base_url: String) -> Self {
        let client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .cookie_store(true)
            .build()
            .unwrap();

        Self { client, base_url }
    }

    pub fn get<T>(&self, endpoint: T) -> RequestBuilder
    where
        T: ToString,
    {
        self.client
            .get(format!("{}{}", self.base_url, endpoint.to_string()))
    }
}
