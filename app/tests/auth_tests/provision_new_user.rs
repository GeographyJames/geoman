use crate::common::{
    AppBuilder, Auth,
    helpers::{assert_ok, handle_json_response},
    services::AuthService,
};
use domain::User;

#[actix_web::test]
pub async fn new_clerk_user_is_added_to_database() {
    let app = AppBuilder::new()
        .set_env(app::enums::GeoManEnvironment::Production)
        .build()
        .await;
    let user_1_auth = Auth::Token(
        app.auth
            .get_test_session_token(&app.api_client.client, &app.test_user_id)
            .await,
    );

    // Verify user doesn't exist yet
    let users: Vec<User> = handle_json_response(
        app.users_service
            .get(&app.api_client, Some(&user_1_auth))
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
            .get_one(&app.api_client, Some(&Auth::Token(user_2_token)), "current")
            .await,
    )
    .await
    .expect("failed to retrieve user");
    // Verify user now exists
    let users: Vec<User> = handle_json_response(
        app.users_service
            .get(&app.api_client, Some(&user_1_auth))
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

#[actix_web::test]
pub async fn existing_clerk_user_is_retrieved_from_database() {
    let app = AppBuilder::new()
        .set_env(app::enums::GeoManEnvironment::Production)
        .build()
        .await;
    let auth = Auth::Token(
        app.auth
            .get_test_session_token(&app.api_client.client, &app.test_user_id)
            .await,
    );
    // New user will be provisioned in database
    let response = app
        .users_service
        .get_one(&app.api_client, Some(&auth), "current")
        .await;
    assert_ok(&response);
    // Now the user is in the database and should be retrieved during request
    let response = app
        .users_service
        .get_one(&app.api_client, Some(&auth), "current")
        .await;
    assert_ok(&response);
}
