use app::enums::ProjectIdentifier;

use crate::common::{TestApp, helpers::check_conformance_declaration_response};

#[actix_web::test]
async fn get_project_conformance_declaration_works() {
    let app = TestApp::spawn_with_db().await;
    let (_, _, project_id) = app.generate_ids().await;
    let response = app
        .ogc_service
        .get_project_conformance_declaration(&app.api_client, &ProjectIdentifier::Id(project_id))
        .await;
    check_conformance_declaration_response(response).await;
}
