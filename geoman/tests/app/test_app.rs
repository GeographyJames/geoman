use std::net::TcpListener;

use geoman::app::{get_config, startup};

use crate::app::services::HttpClient;

pub struct TestApp {
    pub api_client: HttpClient,
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
        let server = startup::run(listener, config).expect("failed to run server");
        let _ = tokio::spawn(server);
        Self { api_client }
    }
}
