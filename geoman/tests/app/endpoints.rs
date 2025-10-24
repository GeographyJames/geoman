use geoman::URLS;
use reqwest::Response;

use crate::app::TestApp;

const REQUEST_FAILED: &str = "failed to execute request";

impl TestApp {
    pub async fn health_check(&self) -> Response {
        self.api_client
            .get(&URLS.health_check)
            .send()
            .await
            .expect(REQUEST_FAILED)
    }

    pub async fn health_check_authenticated(&self, auth_token: Option<&str>) -> Response {
        let mut req = self.api_client.get(&URLS.health_check_authenticated);
        if let Some(token) = auth_token {
            req = req.header("Authorization", token)
        };
        req.send().await.expect(REQUEST_FAILED)
    }
}
