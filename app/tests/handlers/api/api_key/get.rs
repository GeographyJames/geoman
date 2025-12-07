use domain::{ApiKey, User};

use crate::common::{
    AppBuilder, Auth,
    helpers::{assert_ok, handle_json_response},
};

#[actix_web::test]
async fn get_api_keys_works() {
    let app = AppBuilder::new().build().await;

    let response = app
        .api_keys_service
        .get_all(&app.api_client, Some(&Auth::mock_session_token()))
        .await;
    assert_ok(&response);
    let keys: Vec<ApiKey> = handle_json_response(response)
        .await
        .expect("failed to retrieve api keys");
    // keys should be empty as we have not generated any
    assert!(keys.is_empty());
    // Users should ony have a single root user because we have not provisioned a new user
    let users: Vec<User> = handle_json_response(
        app.users_service
            .get(&app.api_client, Some(&Auth::mock_session_token()))
            .await,
    )
    .await
    .expect("failed to retrieve users");
    assert_eq!(users.len(), 1);
    // Now we generate a token
    let _response = app
        .generate_api_key(Some(&Auth::mock_session_token()))
        .await;

    let keys: Vec<ApiKey> = handle_json_response(
        app.api_keys_service
            .get_all(&app.api_client, Some(&Auth::mock_session_token()))
            .await,
    )
    .await
    .expect("failed to retrieve keys");
    assert_eq!(keys.len(), 1);
    // Now users should be two because we have generated a key which would provision the user
    let users: Vec<User> = handle_json_response(
        app.users_service
            .get(&app.api_client, Some(&Auth::mock_session_token()))
            .await,
    )
    .await
    .expect("failed to retrieve users");
    assert_eq!(users.len(), 2);
}
