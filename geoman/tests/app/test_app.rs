use std::net::TcpListener;

use geoman::app::{config::get_config, startup};

use crate::app::{auth::clerk::ClerkAuthProvider, services::HttpClient};

pub struct TestApp {
    pub api_client: HttpClient,
    pub auth: ClerkAuthProvider,
}

impl TestApp {
    pub async fn spawn() -> Self {
        let mut config = get_config().expect("failed to intialise app config");
        let listener = TcpListener::bind(format!("{}:0", config.app_settings.host))
            .expect("failed to bind to port");
        let port = listener.local_addr().unwrap().port();
        config.app_settings.port = port;
        let api_client = HttpClient::new(format!(
            "http://{}:{}",
            config.app_settings.host, config.app_settings.port
        ));
        let server = startup::run(listener, &config).expect("failed to run server");
        let _ = tokio::spawn(server);
        let auth = ClerkAuthProvider {
            secret: config.auth.clerk_secret_key,
        };
        Self { api_client, auth }
    }
    pub async fn get_test_session_token(&self) -> String {
        self.auth
            .get_test_session_token(&self.api_client.client)
            .await
    }
}
