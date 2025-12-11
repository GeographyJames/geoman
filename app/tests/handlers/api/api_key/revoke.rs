use app::enums::GeoManEnvironment;
use domain::ApiKey;

use crate::common::{
    AppBuilder, Auth,
    helpers::{assert_ok, assert_status, handle_json_response},
    services::AuthService,
};

#[actix_web::test]
async fn revoke_api_key_works() {
    let app = AppBuilder::new().build().await;
    let auth = Auth::mock_session_token();
    let key = app.generate_api_key(Some(&auth)).await;
    let keys: Vec<ApiKey> = handle_json_response(
        app.api_keys_service
            .get_all(&app.api_client, Some(&auth))
            .await,
    )
    .await
    .unwrap();
    assert!(keys.iter().any(|k| k.id == key.id));
    let response = app
        .api_keys_service
        .revoke(&app.api_client, key.id, Some(&auth))
        .await;
    assert_status(&response, 204);
    let keys: Vec<ApiKey> = handle_json_response(
        app.api_keys_service
            .get_all(&app.api_client, Some(&auth))
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
    let token = app
        .auth
        .get_test_session_token(&app.api_client.client, &app.test_user_id)
        .await;
    let key = app.generate_api_key(Some(&Auth::Token(token))).await;
    let user_2_token = app
        .auth
        .get_test_session_token(&app.api_client.client, &app.test_user_2_id)
        .await;
    let response = app
        .api_keys_service
        .revoke(&app.api_client, key.id, Some(&Auth::Token(user_2_token)))
        .await;
    assert_status(&response, 404);
}

#[actix_web::test]
async fn revoked_api_key_returns_401() {
    let app = AppBuilder::new()
        .set_env(GeoManEnvironment::Production)
        .build()
        .await;
    let auth = Auth::Token(
        app.auth
            .get_test_session_token(&app.api_client.client, &app.test_user_id)
            .await,
    );
    let key = app.generate_api_key(Some(&auth)).await;
    let response = app
        .ogc_service
        .get_landing_page(&app.api_client, Some(&Auth::Key(key.api_key.clone())))
        .await;
    assert_ok(&response);
    let response = app
        .api_keys_service
        .revoke(&app.api_client, key.id, Some(&auth))
        .await;
    assert_status(&response, 204);
    let response = app
        .ogc_service
        .get_landing_page(&app.api_client, Some(&Auth::Key(key.api_key)))
        .await;
    assert_status(&response, 401);
}
