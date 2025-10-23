use uuid::Uuid;

use crate::{
    app::{TestApp, helpers::assert_ok},
    clerk_auth,
};

#[actix_web::test]
async fn health_check_works() {
    let app = TestApp::spawn().await;
    let response = app.health_check().await;
    assert!(response.status().is_success())
}

#[actix_web::test]
async fn requests_missing_authentication_token_are_rejected() {
    let app = TestApp::spawn().await;
    let response = app.health_check_authenticated(None).await;
    assert_eq!(401, response.status().as_u16())
}

#[actix_web::test]
async fn requests_with_invalid_token_are_rejected() {
    let app = TestApp::spawn().await;
    let response = app
        .health_check_authenticated(Some(&Uuid::new_v4().to_string()))
        .await;
    assert_eq!(401, response.status().as_u16())
}

#[actix_web::test]
async fn requests_with_valid_token_are_accepted() {
    let app = TestApp::spawn().await;
    let token = clerk_auth::get_test_session(
        &app.config.auth.clerk_secret_key,
        "user_34TBak0wKXjYNSdz8EsCnCTrlVY",
    )
    .await;
    let response = app.health_check_authenticated(Some(&token.jwt)).await;
    assert_ok(&response);
}
