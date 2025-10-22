use std::net::TcpListener;

use crate::app::services::HttpClient;

pub struct TestApp {
    pub _address: String,
    pub api_client: HttpClient,
}

impl TestApp {
    pub async fn spawn() -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind to random port");
        let port = listener.local_addr().unwrap().port();
        let server = geoman::run(listener).expect("failed to run server");
        let _ = tokio::spawn(server);
        let address = format!("http://127.0.0.1:{}", port);
        let api_client = HttpClient::new(address.clone());
        Self {
            _address: address,
            api_client,
        }
    }
}
