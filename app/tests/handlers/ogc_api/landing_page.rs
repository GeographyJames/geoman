use crate::common::{
    TestApp,
    helpers::{assert_ok, assert_status, handle_json_response},
};
use app::enums::ProjectIdentifier;
use domain::ProjectId;
use ogc::types::common::LandingPage;
use reqwest::Response;

#[actix_web::test]
async fn get_landing_page_works() {
    let app = TestApp::spawn().await;
    let response = app.ogc_service.get_landing_page(&app.api_client).await;
    check_landing_page_response(response).await;
}

#[actix_web::test]
async fn get_project_landing_page_works() {
    let app = TestApp::spawn_with_db().await;
    let (_, _, project_id) = app.generate_ids().await;

    let response = app
        .ogc_service
        .get_project_landing_page(&app.api_client, ProjectIdentifier::Id(project_id))
        .await;
    check_landing_page_response(response).await;
}

#[actix_web::test]
async fn get_project_landing_page_returns_404_for_project_not_found() {
    let app = TestApp::spawn_with_db().await;
    let response = app
        .ogc_service
        .get_project_landing_page(&app.api_client, ProjectIdentifier::Id(ProjectId(0)))
        .await;
    assert_status(&response, 404);
}

async fn check_landing_page_response(response: Response) {
    assert_ok(&response);
    let _landing_page: LandingPage = handle_json_response(response)
        .await
        .expect("failed to retrieve landing page");
}
