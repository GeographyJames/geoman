use app::enums::ProjectIdentifier;
use domain::ProjectId;
use reqwest::Response;

use crate::common::{
    TestApp,
    helpers::{assert_ok, assert_status, handle_json_response},
};

#[actix_web::test]
async fn get_conformance_declaration_works() {
    let app = TestApp::spawn().await;

    let response = app
        .ogc_service
        .get_conformance_declaration(&app.api_client)
        .await;

    check_conformance_declaration_response(response).await
}

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

#[actix_web::test]
async fn get_project_conformance_declaration_returns_404_for_project_not_found() {
    let app = TestApp::spawn_with_db().await;
    let response = app
        .ogc_service
        .get_project_conformance_declaration(&app.api_client, &ProjectIdentifier::Id(ProjectId(0)))
        .await;
    assert_status(&response, 404);
}

async fn check_conformance_declaration_response(response: Response) {
    assert_ok(&response);
    let _conformance: ogc::ConformanceDeclaration = handle_json_response(response)
        .await
        .expect("failed to retrieve conformance");
}
