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

#[actix_web::test]
async fn requests_missing_authentication_are_rejected() {
    let app = TestApp::spawn().await;
    let response = app
        .api_client
        .get(&URLS.health_check_authenticated)
        .send()
        .await
        .expect("failed to execute request");
    assert_eq!(401, response.status().as_u16())
}
