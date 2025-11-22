use ogcapi_types::common::Conformance;

use crate::common::{
    TestApp,
    helpers::{assert_ok, handle_json_response},
};

#[actix_web::test]
async fn get_conformance_declaration_works() {
    let app = TestApp::spawn().await;

    let response = app
        .ogc_service
        .get_conformance_declaration(&app.api_client)
        .await;

    assert_ok(&response);
    handle_json_response::<Conformance>(response)
        .await
        .expect("failed to retrieve conformance");
}
