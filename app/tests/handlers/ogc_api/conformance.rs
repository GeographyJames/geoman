use crate::common::{TestApp, helpers::check_conformance_declaration_response};

#[actix_web::test]
async fn get_conformance_declaration_works() {
    let app = TestApp::spawn().await;

    let response = app
        .ogc_service
        .get_conformance_declaration(&app.api_client)
        .await;

    check_conformance_declaration_response(response).await
}
