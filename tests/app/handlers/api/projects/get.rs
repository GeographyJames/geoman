use geoman::domain::Project;

use crate::app::{TestApp, helpers::handle_json_response};

#[actix_web::test]
async fn get_projects_works() {
    let app = TestApp::spawn().await;
    let token = app.get_test_session_token().await;
    let response = app
        .projects_service
        .get(&app.api_client, Some(&token))
        .await;
    let projects: Vec<Project> = handle_json_response(response)
        .await
        .expect("failed to retrieve projects");
    assert!(!projects.is_empty())
}
