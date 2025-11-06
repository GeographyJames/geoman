use geoman::ogc::types::common::Collections;

use crate::app::{
    TestApp,
    helpers::{assert_ok, handle_json_response},
};

#[actix_web::test]
async fn get_collections_works() {
    let app = TestApp::spawn().await;

    let response = app.ogc_service.get_collections(&app.api_client).await;

    assert_ok(&response);

    let _collections: Collections = handle_json_response(response)
        .await
        .expect("failed to retrieve collections");
}
