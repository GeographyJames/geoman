use actix_web::App;
use app::{ErrorResponse, handlers::api::projects::PostProjectPayload};
use domain::{ProjectId, TechnologyId, project::Project};

use crate::common::{
    AppBuilder, Auth,
    helpers::{check_error_response, handle_json_response},
};

#[tokio::test]
async fn post_project_works() {
    let app = AppBuilder::new().build().await;
    let _project_id = app
        .generate_project_id(Some(&Auth::mock_session_token()))
        .await;
}

#[tokio::test]
async fn post_project_returns_409_for_duplicate_name() {
    let app = AppBuilder::new().build().await;
    let project = PostProjectPayload::default();
    let _id: ProjectId = handle_json_response(
        app.projects_service
            .post_json(&app.api_client, Some(&Auth::mock_session_token()), &project)
            .await,
    )
    .await
    .expect("failed to get project id");
    let response = app
        .projects_service
        .post_json(&app.api_client, Some(&Auth::mock_session_token()), &project)
        .await;
    let err: ErrorResponse = check_error_response(response, 409).await;
    assert!(
        err.message
            .to_lowercase()
            .contains("a project with this name")
    )
}

#[tokio::test]
async fn post_project_works_with_technologies() {
    let app = AppBuilder::new().build().await;
    let mut project = PostProjectPayload::default();
    project.technologies = Some(vec![TechnologyId(1)]);
    let id: ProjectId = handle_json_response(
        app.projects_service
            .post_json(&app.api_client, Some(&Auth::mock_session_token()), &project)
            .await,
    )
    .await
    .unwrap();
    let response = app
        .ogc_service
        .get_feature(&app.api_client, "projects", id.0)
        .await;
    let ogc_feature: ogc::Feature = handle_json_response(response)
        .await
        .expect("failed to retrieve project");

    let project = Project::try_from(ogc_feature).expect("Failed to convert to project");
    assert!(!project.properties.technologies.is_empty())
}
