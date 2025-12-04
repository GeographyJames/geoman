use actix_web::App;
use app::handlers::api::projects::ProjectReqPayload;
use domain::ProjectId;

use crate::common::{AppBuilder, helpers::handle_json_response};

#[tokio::test]
async fn post_project_works() {
    let app = AppBuilder::new().build().await;
    let token = app.generate_session_token().await;
    let response = app
        .projects_service
        .post_json(&app.api_client, Some(&token), &ProjectReqPayload::default())
        .await;
    let _project_id: ProjectId = handle_json_response(response)
        .await
        .expect("failed to retrieve project id");
}

#[tokio::test]
async fn post_project_returns_409_for_duplicate_name() {
    let app = AppBuilder::new().build().await;
}
