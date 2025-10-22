use geoman::URLS;

use crate::app::TestApp;

#[actix_web::test]
async fn health_check_works() {
    let app = TestApp::spawn().await;
    let response = app
        .api_client
        .get(&URLS.health_check)
        .send()
        .await
        .expect("failed to execute request");
    assert!(response.status().is_success())
}
