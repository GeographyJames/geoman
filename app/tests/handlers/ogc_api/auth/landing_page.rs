use app::enums::GeoManEnvironment;

use crate::common::{
    AppBuilder,
    helpers::{assert_ok, assert_status},
    services::{OgcAuth, SessionToken},
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
            Some(OgcAuth::Token(SessionToken(
                uuid::Uuid::new_v4().to_string(),
            ))),
        )
        .await;
    assert_status(&response, 401);

    // Requests with valid token are accepted
    let token = app.generate_session_token().await;

    let response = app
        .ogc_service
        .get_landing_page(&app.api_client, Some(OgcAuth::Token(token.clone())))
        .await;
    assert_ok(&response);

    // Requests with invalid api key are rejected
    let response = app
        .ogc_service
        .get_landing_page(
            &app.api_client,
            Some(OgcAuth::Key(uuid::Uuid::new_v4().to_string())),
        )
        .await;
    assert_status(&response, 401);

    // Requests with valid api key are accepted
    let api_key = app.generate_api_key(&token).await;
    let response = app
        .ogc_service
        .get_landing_page(&app.api_client, Some(OgcAuth::Key(api_key.api_key)))
        .await;
    assert_ok(&response);
}
