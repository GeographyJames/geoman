use app::handlers::api::projects::PatchProjectPayload;
use domain::{
    enums::{CollectionId, Status},
    project::Project,
};

use crate::common::{
    AppBuilder, Auth,
    helpers::{assert_status, handle_json_response},
};

#[actix_web::test]
async fn patch_project_works() {
    let app = AppBuilder::new().build().await;
    let project_id = app
        .generate_project_id(Some(&Auth::mock_session_token()))
        .await;
    let mut updated_project = PatchProjectPayload::default();
    updated_project.status = Some(Status::Archived);
    let response = app
        .projects_service
        .patch_json(
            &app.api_client,
            project_id,
            Some(&Auth::mock_session_token()),
            &updated_project,
        )
        .await;
    assert_status(&response, 204);
    let project: ogc::Feature = handle_json_response(
        app.ogc_service
            .get_feature(
                &app.api_client,
                CollectionId::Projects.to_string().as_str(),
                project_id.0,
            )
            .await,
    )
    .await
    .expect("failed to retrieve project");
    let project = Project::try_from(project).unwrap();
    assert_eq!(project.properties.status, Status::Archived);
}
