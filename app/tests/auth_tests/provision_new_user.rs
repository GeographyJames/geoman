use crate::common::{AppBuilder, helpers::handle_json_response, services::AuthService};
use domain::User;

#[actix_web::test]
pub async fn new_clerk_user_is_added_to_database() {
    let app = AppBuilder::new()
        .set_env(app::enums::GeoManEnvironment::Production)
        .build()
        .await;
    let user_1_token = app
        .auth
        .get_test_session_token(&app.api_client.client, &app.test_user_id)
        .await;

    // Verify user doesn't exist yet
    let users: Vec<User> = handle_json_response(
        app.users_service
            .get(&app.api_client, Some(&user_1_token))
            .await,
    )
    .await
    .expect("failed to retrieve users");
    assert!(
        !users
            .iter()
            .any(|user| user.clerk_id.as_ref() == Some(&app.test_user_2_id)),
        "the user should not be in the database"
    );

    let user_2_token = app
        .auth
        .get_test_session_token(&app.api_client.client, &app.test_user_2_id)
        .await;
    let _user: User = handle_json_response(
        app.users_service
            .get_one(&app.api_client, Some(&user_2_token), "current")
            .await,
    )
    .await
    .expect("failed to retrieve user");
    // Verify user now exists
    let users: Vec<User> = handle_json_response(
        app.users_service
            .get(&app.api_client, Some(&user_1_token))
            .await,
    )
    .await
    .expect("failed to retrieve users");
    assert!(
        users
            .iter()
            .any(|user| user.clerk_id.as_ref() == Some(&app.test_user_2_id)),
        "the user should now be in the database"
    );
}
