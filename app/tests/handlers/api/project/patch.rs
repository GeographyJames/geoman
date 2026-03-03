use app::{
    constants::UNASSIGNED_USERS_TEAM_ID,
    handlers::api::{projects::PatchProjectPayload, users::PatchUserPayload},
};
use domain::{
    ProjectId, TeamId,
    enums::{CollectionId, Status},
    project::Project,
};

use crate::common::{
    AppBuilder, Auth, TestApp,
    helpers::{assert_status, handle_json_response},
};

#[actix_web::test]
async fn patch_project_works() {
    let (app, user, project_id) = TestApp::with_project().await;

    let mut updated_project = PatchProjectPayload::default();
    updated_project.status = Some(Status::Archived);
    let response = app
        .projects_service
        .patch_json(&app.api_client, project_id, Some(&user), &updated_project)
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

#[actix_web::test]
async fn only_project_owner_and_admins_can_delete_project() {
    let (app, project_owner, project_id) = TestApp::with_project().await;
    let update_dto = PatchProjectPayload {
        status: Some(Status::Deleted),
        ..Default::default()
    };
    let user_2 = Auth::MockUserCredentials(app.generate_user(false, TeamId(0)).await);
    let response = app
        .projects_service
        .patch_json(&app.api_client, project_id, Some(&user_2), &update_dto)
        .await;
    assert_eq!(
        response.status().as_u16(),
        401,
        "non admin non project owner should not be able to delete project"
    );
    let admin_user = Auth::MockUserCredentials(app.generate_user(true, TeamId(0)).await);
    let response = app
        .projects_service
        .patch_json(&app.api_client, project_id, Some(&admin_user), &update_dto)
        .await;
    assert_eq!(
        response.status().as_u16(),
        204,
        "admin should be able to delete project"
    );
    let project_id = app.generate_project_id(Some(&project_owner)).await;
    let response = app
        .projects_service
        .patch_json(
            &app.api_client,
            project_id,
            Some(&project_owner),
            &update_dto,
        )
        .await;
    assert_eq!(
        response.status().as_u16(),
        204,
        "project owner should be able to delete project"
    );
}

#[actix_web::test]
async fn patch_project_privileges_check() {
    let app = AppBuilder::new().build().await;
    let admin_user = Auth::MockUserCredentials(app.generate_user(true, TeamId(0)).await);
    let project_owner_team = app.generate_team_id(Some(&admin_user)).await;
    let project_owner = app.generate_user(false, project_owner_team).await;
    let project_owner_id = project_owner.id.0;
    let project_owner = Auth::MockUserCredentials(project_owner);
    let project_id = app.generate_project_id(Some(&project_owner)).await;
    let second_team = app.generate_team_id(Some(&(admin_user))).await;
    let user_on_another_team =
        Auth::MockUserCredentials(app.generate_user(false, second_team).await);
    let update_dto = PatchProjectPayload::default();
    let response = app
        .projects_service
        .patch_json(
            &app.api_client,
            project_id,
            Some(&user_on_another_team),
            &update_dto,
        )
        .await;
    assert_eq!(
        response.status().as_u16(),
        401,
        "User on another team should not be able to update project"
    );
    let response = app
        .projects_service
        .patch_json(&app.api_client, project_id, Some(&admin_user), &update_dto)
        .await;
    assert_eq!(
        response.status().as_u16(),
        204,
        "admin on another team should be able to edit project"
    );
    let teammate = Auth::MockUserCredentials(app.generate_user(false, project_owner_team).await);
    let response = app
        .projects_service
        .patch_json(&app.api_client, project_id, Some(&teammate), &update_dto)
        .await;
    assert_eq!(
        response.status().as_u16(),
        204,
        "teammate should be able to update project"
    );
    // change owner team to Unassigned users
    let updated_owner = PatchUserPayload {
        team_id: Some(TeamId(UNASSIGNED_USERS_TEAM_ID)),
        admin: None,
    };
    let response = app
        .users_service
        .patch_json(
            &app.api_client,
            project_owner_id,
            Some(&admin_user),
            &updated_owner,
        )
        .await;
    assert_status(&response, 204);
    let unassigned_user = Auth::MockUserCredentials(
        app.generate_user(false, TeamId(UNASSIGNED_USERS_TEAM_ID))
            .await,
    );
    let response = app
        .projects_service
        .patch_json(
            &app.api_client,
            project_id,
            Some(&unassigned_user),
            &update_dto,
        )
        .await;
    assert_eq!(
        response.status().as_u16(),
        401,
        "unassigned users should not be able to update projects"
    );
}

#[actix_web::test]
async fn patch_nonexistent_project_returns_404() {
    let (app, user, _) = TestApp::with_project().await;
    let update_dto = PatchProjectPayload::default();
    let response = app
        .projects_service
        .patch_json(
            &app.api_client,
            ProjectId(i32::MAX),
            Some(&user),
            &update_dto,
        )
        .await;
    assert_status(&response, 404);
}
