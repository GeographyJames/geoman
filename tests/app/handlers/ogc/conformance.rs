use geoman::ogc::types::common::ConformanceDeclaration;

use crate::app::{TestApp, helpers::handle_json_response};

#[actix_web::test]
async fn get_conformance_declaration_works() {
    let app = TestApp::spawn().await;

    let response = app
        .ogc_service
        .get_conformance_declaration(&app.api_client)
        .await;

    let _conformance: ConformanceDeclaration = handle_json_response(response)
        .await
        .expect("failed to retrieve conformance");
}
