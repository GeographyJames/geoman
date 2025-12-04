use domain::ApiKey;

use crate::common::{
    AppBuilder,
    helpers::{assert_status, handle_json_response},
};

#[actix_web::test]
async fn renew_api_key_works() {
    let app = AppBuilder::new().build().await;
    let token = app.generate_session_token().await;
    let api_key = app.generate_api_key(None).await;
    let keys: Vec<ApiKey> =
        handle_json_response(app.api_keys_service.get_all(&app.api_client, None).await)
            .await
            .unwrap();
    let expiry = keys.iter().find(|k| k.id == api_key.id).unwrap().expiry;
    let response = app
        .api_keys_service
        .renew(&app.api_client, api_key.id, None)
        .await;
    assert_status(&response, 204);
    let keys: Vec<ApiKey> =
        handle_json_response(app.api_keys_service.get_all(&app.api_client, None).await)
            .await
            .unwrap();
    let updated_expiry = keys.iter().find(|k| k.id == api_key.id).unwrap().expiry;
    assert!(updated_expiry > expiry);
}
