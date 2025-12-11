use domain::ApiKey;

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
}
