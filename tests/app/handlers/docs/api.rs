use crate::app::{
    TestApp,
    helpers::{assert_ok, assert_status},
};

#[actix_web::test]
async fn get_api_docs_works() {
    let app = TestApp::spawn().await;
    let token = app.get_test_session_token().await;
    let response = app
        .api_docs_service
        .get(&app.api_client, Some(&token))
        .await;
    assert_ok(&response);
}

#[actix_web::test]
async fn get_api_docs_returns_401_for_unauthenticated() {
    let app = TestApp::spawn().await;
    let response = app.api_docs_service.get(&app.api_client, None).await;
    assert_status(&response, 401)
}
