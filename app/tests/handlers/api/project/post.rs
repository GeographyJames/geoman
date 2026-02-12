use app::{ErrorResponse, handlers::api::projects::PostProjectPayload};
use domain::ProjectId;

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
