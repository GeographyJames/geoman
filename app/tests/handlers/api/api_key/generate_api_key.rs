use crate::common::{
    AppBuilder,
    helpers::{assert_ok, handle_json_response},
};
use app::handlers::api::keys::ResponsePayload;

#[actix_web::test]
async fn generate_api_key_works() {
    let app = AppBuilder::new().build().await;
    let token = app.generate_clerk_session_token().await;
    let response = app
        .api_keys_service
        .generate_api_key(&app.api_client, Some(&token))
        .await;
    assert_ok(&response);
    let _key: ResponsePayload = handle_json_response(response)
        .await
        .expect("failed to retrive key");
}
