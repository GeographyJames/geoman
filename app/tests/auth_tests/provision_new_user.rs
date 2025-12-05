use crate::common::{
    AppBuilder,
    helpers::assert_ok,
    services::{AuthService, OgcAuth},
};
use domain::UserId;

#[actix_web::test]
pub async fn new_clerk_user_is_added_to_database() {
    let app = AppBuilder::new()
        .set_env(app::enums::GeoManEnvironment::Production)
        .build()
        .await;

    // Verify user doesn't exist yet
    let user_before = sqlx::query_scalar!(
        r#"SELECT id AS "id: UserId" FROM app.users WHERE clerk_id = $1"#,
        app.test_user_id
    )
    .fetch_optional(&app.db_pool)
    .await
    .expect("Failed to query database");

    assert!(
        user_before.is_none(),
        "User should not exist in database yet"
    );

    let token = app
        .auth
        .get_test_session_token(&app.api_client.client, &app.test_user_id)
        .await;
    let response = app
        .ogc_service
        .get_landing_page(&app.api_client, Some(&OgcAuth::Token(token)))
        .await;
    assert_ok(&response);

    // Verify user was auto-provisioned in the database
    let user_after = sqlx::query_scalar!(
        r#"SELECT id AS "id: UserId" FROM app.users WHERE clerk_id = $1"#,
        app.test_user_id
    )
    .fetch_optional(&app.db_pool)
    .await
    .expect("Failed to query database");

    assert!(
        user_after.is_some(),
        "User should have been auto-provisioned in database"
    );
}
