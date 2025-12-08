use domain::User;

use crate::common::{
    AppBuilder, Auth,
    helpers::{assert_ok, handle_json_response},
};

#[tokio::test]
async fn get_users_works() {
    let app = AppBuilder::new().build().await;
    let response = app
        .users_service
        .get(&app.api_client, Some(&Auth::mock_session_token()))
        .await;
    assert_ok(&response);
    let _users: Vec<User> = handle_json_response(response)
        .await
        .expect("failed to retrieve users");
}

#[tokio::test]
async fn get_current_user_works() {
    let app = AppBuilder::new().build().await;
    let response = app
        .users_service
        .get_one(
            &app.api_client,
            Some(&Auth::mock_session_token()),
            "current",
        )
        .await;
    assert_ok(&response);
    let retreived_user: User = handle_json_response(response)
        .await
        .expect("failed to retrieve user");
    assert_eq!(retreived_user.first_name, "root");
}
