use std::net::TcpListener;

use geoman::URLS;

pub struct TestApp {
    pub address: String,
}

impl TestApp {
    pub async fn spawn() -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind to random port");
        let port = listener.local_addr().unwrap().port();
        let server = geoman::run(listener).expect("failed to run server");
        let _ = tokio::spawn(server);
        Self {
            address: format!("http://127.0.0.1:{}", port),
        }
    }
}

#[actix_web::test]
async fn health_check_works() {
    let app = TestApp::spawn().await;
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}{}", &app.address, URLS.health_check))
        .send()
        .await
        .expect("failed to execute request");
    assert!(response.status().is_success())
}
