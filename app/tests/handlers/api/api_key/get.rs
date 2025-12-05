use domain::ApiKey;

use crate::common::{
    AppBuilder,
    helpers::{assert_ok, handle_json_response},
};

#[actix_web::test]
async fn get_api_keys_works() {
    let app = AppBuilder::new().build().await;

    let response = app.api_keys_service.get_all(&app.api_client, None).await;
    assert_ok(&response);
    let _keys: Vec<ApiKey> = handle_json_response(response)
        .await
        .expect("failed to retrieve api keys");
}
