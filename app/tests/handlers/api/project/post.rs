use app::handlers::api::projects::ProjectReqPayload;
use domain::ProjectId;

use crate::common::{AppBuilder, helpers::handle_json_response};

#[tokio::test]
async fn post_project_works() {
    let app = AppBuilder::new().build().await;
    let token = app.generate_session_token().await;
    let response = app
        .projects_service
        .post_json(
            &app.api_client,
            Some(&token),
            &ProjectReqPayload {
                name: uuid::Uuid::new_v4().to_string(),
                visibility: domain::enums::Visibility::Public,
                country_code: "GB".to_string(),
                crs_srid: None,
            },
        )
        .await;

    let _project_id: ProjectId = handle_json_response(response)
        .await
        .expect("failed to retrieve project id");
}
