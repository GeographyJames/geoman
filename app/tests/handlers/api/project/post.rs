use app::{ErrorResponse, handlers::api::projects::ProjectReqPayload};
use domain::ProjectId;

use crate::common::{
    AppBuilder,
    helpers::{check_error_response, handle_json_response},
};

#[tokio::test]
async fn post_project_works() {
    let app = AppBuilder::new().build().await;
    let response = app
        .projects_service
        .post_json(&app.api_client, None, &ProjectReqPayload::default())
        .await;
    let _project_id: ProjectId = handle_json_response(response)
        .await
        .expect("failed to retrieve project id");
}

#[tokio::test]
async fn post_project_returns_409_for_duplicate_name() {
    let app = AppBuilder::new().build().await;
    let project = ProjectReqPayload::default();
    let _id: ProjectId = handle_json_response(
        app.projects_service
            .post_json(&app.api_client, None, &project)
            .await,
    )
    .await
    .expect("failed to get project id");
    let response = app
        .projects_service
        .post_json(&app.api_client, None, &project)
        .await;
    let err: ErrorResponse = check_error_response(response, 409).await;
    assert!(
        err.message
            .to_lowercase()
            .contains("a project with this name")
    )
}
