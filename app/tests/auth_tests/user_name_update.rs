use crate::common::{AppBuilder, helpers::handle_json_response, services::AuthService};
use clerk_rs::{
    ClerkConfiguration,
    clerk::Clerk,
    models::{CreateUserRequest, UpdateUserRequest, User},
};

use secrecy::ExposeSecret;
#[actix_web::test]
async fn user_name_updated_in_database_on_name_change() {
    let app = AppBuilder::new()
        .set_env(app::enums::GeoManEnvironment::Production)
        .build()
        .await;
    let first_name = uuid::Uuid::new_v4().to_string();
    let last_name = uuid::Uuid::new_v4().to_string();
    let password = uuid::Uuid::new_v4().to_string();
    let clerk_config = ClerkConfiguration::new(
        None,
        None,
        Some(
            app.app_config
                .auth_settings
                .clerk_secret_key
                .expose_secret()
                .clone(),
        ),
        None,
    );
    let clerk = Clerk::new(clerk_config);
    let new_user =
        create_clerk_test_user(&clerk, first_name.clone(), last_name.clone(), password).await;
    let id = new_user.id.expect("user has no id");
    let token = app
        .auth
        .get_test_session_token(&app.api_client.client, &id)
        .await;
    let user: domain::User = handle_json_response(
        app.users_service
            .get_one(&app.api_client, Some(&token), "current")
            .await,
    )
    .await
    .expect("failed to retrieve user");
    assert_eq!(user.first_name, first_name);
    assert_eq!(user.last_name, last_name);

    let new_first_name = uuid::Uuid::new_v4().to_string();
    let new_last_name = uuid::Uuid::new_v4().to_string();
    update_clerk_user(&clerk, &id, new_first_name.clone(), new_last_name.clone()).await;

    // Update token
    let token = app
        .auth
        .get_test_session_token(&app.api_client.client, &id)
        .await;
    // Move this to some kind of cleanup at end of function.
    delete_clerk_user(&clerk, &id).await;

    let updated_user: domain::User = handle_json_response(
        app.users_service
            .get_one(&app.api_client, Some(&token), "current")
            .await,
    )
    .await
    .expect("failed to retrieve user");
    assert_eq!(updated_user.id, user.id);
    assert_eq!(updated_user.first_name, new_first_name);
    assert_eq!(updated_user.last_name, new_last_name);
}

async fn create_clerk_test_user(
    clerk: &Clerk,
    first_name: String,
    last_name: String,
    password: String,
) -> User {
    let mut create_user_request = CreateUserRequest::new();
    create_user_request.first_name = Some(Some(first_name));
    create_user_request.last_name = Some(Some(last_name));
    create_user_request.password = Some(Some(password));

    let user = clerk_rs::apis::users_api::User::create_user(clerk, create_user_request)
        .await
        .inspect_err(|e| {
            eprintln!("Clerk API Error Details:");
            eprintln!("{:#?}", e);
        })
        .expect("failed to create clerk user");
    user
}

async fn update_clerk_user(clerk: &Clerk, user_id: &str, first_name: String, last_name: String) {
    let mut req = UpdateUserRequest::new();
    req.first_name = Some(Some(first_name));
    req.last_name = Some(Some(last_name));
    clerk_rs::apis::users_api::User::update_user(clerk, user_id, req)
        .await
        .expect("failed to update user first and last name");
}

async fn delete_clerk_user(clerk: &Clerk, user_id: &str) {
    clerk_rs::apis::users_api::User::delete_user(clerk, user_id)
        .await
        .expect("failed to delete user");
}
