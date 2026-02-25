use app::handlers::api::users::PatchUserPayload;
use domain::TeamId;

use crate::common::{AppBuilder, Auth, helpers::assert_status};

#[tokio::test]
async fn patch_user_admin_works() {
    let app = AppBuilder::new().build().await;
    let admin_user = Auth::MockUserCredentials(app.generate_user(true, TeamId(-1)).await);
    let user_to_update = app.generate_user(false, TeamId(-1)).await;
    let response = app
        .users_service
        .patch_json(
            &app.api_client,
            user_to_update.id.0,
            Some(&admin_user),
            &PatchUserPayload {
                team_id: None,
                admin: Some(true),
            },
        )
        .await;
    assert_status(&response, 204);
}

#[tokio::test]
async fn patch_user_revoke_admin_works() {
    let app = AppBuilder::new().build().await;
    let admin_user = Auth::MockUserCredentials(app.generate_user(true, TeamId(-1)).await);
    let user_to_update = app.generate_user(true, TeamId(-1)).await;
    let response = app
        .users_service
        .patch_json(
            &app.api_client,
            user_to_update.id.0,
            Some(&admin_user),
            &PatchUserPayload {
                team_id: None,
                admin: Some(false),
            },
        )
        .await;
    assert_status(&response, 204);
}

#[tokio::test]
async fn patch_user_works() {
    let app = AppBuilder::new().build().await;
    let admin_user = Auth::MockUserCredentials(app.generate_user(true, TeamId(-1)).await);
    let team_id = app.generate_team_id(Some(&admin_user)).await;
    let user_to_update = app.generate_user(false, team_id).await;
    let response = app
        .users_service
        .patch_json(
            &app.api_client,
            user_to_update.id.0,
            Some(&admin_user),
            &PatchUserPayload {
                team_id: Some(team_id),
                admin: None,
            },
        )
        .await;
    assert_status(&response, 204);
}

#[tokio::test]
async fn patch_user_requires_admin_permission() {
    let app = AppBuilder::new().build().await;
    let admin_user = Auth::MockUserCredentials(app.generate_user(true, TeamId(-1)).await);
    let non_admin_user = Auth::MockUserCredentials(app.generate_user(false, TeamId(-1)).await);

    let team_id = app.generate_team_id(Some(&admin_user)).await;
    let user_to_update = app.generate_user(false, team_id).await;
    let response = app
        .users_service
        .patch_json(
            &app.api_client,
            user_to_update.id.0,
            Some(&non_admin_user),
            &PatchUserPayload {
                team_id: Some(team_id),
                admin: None,
            },
        )
        .await;
    assert_status(&response, 401);
}
