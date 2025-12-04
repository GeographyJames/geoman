use app::enums::GeoManEnvironment;
use domain::ApiKey;

use crate::common::{
    AppBuilder,
    helpers::{assert_ok, assert_status, handle_json_response},
    services::{AuthService, OgcAuth},
};

#[actix_web::test]
async fn revoke_api_key_works() {
    let app = AppBuilder::new().build().await;
    let token = app.generate_session_token().await;
    let key = app.generate_api_key(Some(&token)).await;
    let keys: Vec<ApiKey> = handle_json_response(
        app.api_keys_service
            .get_all(&app.api_client, Some(&token))
            .await,
    )
    .await
    .unwrap();
    assert!(keys.iter().any(|k| k.id == key.id));
    let response = app
        .api_keys_service
        .revoke(&app.api_client, key.id, Some(&token))
        .await;
    assert_status(&response, 204);
    let keys: Vec<ApiKey> = handle_json_response(
        app.api_keys_service
            .get_all(&app.api_client, Some(&token))
            .await,
    )
    .await
    .unwrap();
    assert!(!keys.iter().any(|k| k.id == key.id));
}

#[actix_web::test]
async fn revoke_api_key_returns_404_when_revoking_another_users_key() {
    let app = AppBuilder::new()
        .set_env(GeoManEnvironment::Production)
        .build()
        .await;
    let token = app.generate_session_token().await;
    let key = app.generate_api_key(Some(&token)).await;
    let user_2_token = app
        .auth
        .get_test_session_token(&app.api_client.client, &app.test_user_2_id)
        .await;
    let response = app
        .api_keys_service
        .revoke(&app.api_client, key.id, Some(&user_2_token))
        .await;
    assert_status(&response, 404);
}

#[actix_web::test]
async fn revoked_api_key_returns_401() {
    let app = AppBuilder::new()
        .set_env(GeoManEnvironment::Production)
        .build()
        .await;
    let token = app.generate_session_token().await;
    let key = app.generate_api_key(Some(&token)).await;
    let response = app
        .ogc_service
        .get_landing_page(&app.api_client, Some(&OgcAuth::Key(key.api_key.clone())))
        .await;
    assert_ok(&response);
    app.api_keys_service
        .revoke(&app.api_client, key.id, Some(&token))
        .await;
    let response = app
        .ogc_service
        .get_landing_page(&app.api_client, Some(&OgcAuth::Key(key.api_key)))
        .await;
    assert_status(&response, 401);
}
