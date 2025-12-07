use domain::ApiKey;

use crate::common::{
    AppBuilder, Auth,
    helpers::{assert_status, handle_json_response},
};

#[actix_web::test]
async fn renew_api_key_works() {
    let app = AppBuilder::new().build().await;
    let auth = Auth::mock_session_token();
    let api_key = app.generate_api_key(Some(&auth)).await;
    let keys: Vec<ApiKey> =
        handle_json_response(app.api_keys_service.get_all(&app.api_client, Some(&auth)).await)
            .await
            .unwrap();
    let expiry = keys.iter().find(|k| k.id == api_key.id).unwrap().expiry;
    let response = app
        .api_keys_service
        .renew(&app.api_client, api_key.id, Some(&auth))
        .await;
    assert_status(&response, 204);
    let keys: Vec<ApiKey> =
        handle_json_response(app.api_keys_service.get_all(&app.api_client, Some(&auth)).await)
            .await
            .unwrap();
    let updated_expiry = keys.iter().find(|k| k.id == api_key.id).unwrap().expiry;
    assert!(updated_expiry > expiry);
}
