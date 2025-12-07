use app::enums::GeoManEnvironment;

use crate::common::{
    AppBuilder, Auth,
    helpers::{assert_ok, assert_status},
    services::{AuthService, SessionToken},
};

#[actix_web::test]
pub async fn landing_page_requires_authentication_in_production() {
    let app = AppBuilder::new()
        .set_env(GeoManEnvironment::Production)
        .build()
        .await;

    // Requests without authorisation are rejected
    let response = app
        .ogc_service
        .get_landing_page(&app.api_client, None)
        .await;
    assert_status(&response, 401);

    // Requests with invalid session token are rejected
    let response = app
        .ogc_service
        .get_landing_page(
            &app.api_client,
            Some(&Auth::Token(SessionToken(uuid::Uuid::new_v4().to_string()))),
        )
        .await;
    assert_status(&response, 401);

    // Requests with valid token are accepted
    let token = app
        .auth
        .get_test_session_token(&app.api_client.client, &app.test_user_id)
        .await;

    let response = app
        .ogc_service
        .get_landing_page(&app.api_client, Some(&Auth::Token(token.clone())))
        .await;
    assert_ok(&response);

    // Requests with invalid api key are rejected
    let response = app
        .ogc_service
        .get_landing_page(
            &app.api_client,
            Some(&Auth::Key(uuid::Uuid::new_v4().to_string())),
        )
        .await;
    assert_status(&response, 401);

    // Requests with valid api key are accepted
    let api_key = app.generate_api_key(Some(&Auth::Token(token))).await;
    let response = app
        .ogc_service
        .get_landing_page(&app.api_client, Some(&Auth::Key(api_key.api_key)))
        .await;
    assert_ok(&response);
}
