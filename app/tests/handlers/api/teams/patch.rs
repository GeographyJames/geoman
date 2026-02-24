use app::handlers::api::teams::TeamUpdatePayload;
use domain::TeamId;

use crate::common::{AppBuilder, Auth, helpers::assert_status};

#[actix_web::test]
async fn patch_team_works() {
    let app = AppBuilder::new().build().await;
    let admin_user = Auth::MockUserCredentials(app.generate_user(true, TeamId(0)).await);
    let bu_id = app.generate_bu_id(Some(&admin_user)).await;
    let team_id = app.generate_team_id(Some(&admin_user), bu_id).await;
    let response = app
        .teams_service
        .patch_json(
            &app.api_client,
            team_id.0,
            Some(&admin_user),
            &TeamUpdatePayload {
                business_unit: None,
                name: Some(uuid::Uuid::new_v4().to_string()),
            },
        )
        .await;
    assert_status(&response, 204);
}

#[actix_web::test]
async fn patch_team_requires_admin_permission() {
    let app = AppBuilder::new().build().await;
    let admin_user = Auth::MockUserCredentials(app.generate_user(true, TeamId(0)).await);
    let non_admin_user = Auth::MockUserCredentials(app.generate_user(false, TeamId(-1)).await);
    let bu_id = app.generate_bu_id(Some(&admin_user)).await;
    let team_id = app.generate_team_id(Some(&admin_user), bu_id).await;
    let response = app
        .teams_service
        .patch_json(
            &app.api_client,
            team_id.0,
            Some(&non_admin_user),
            &TeamUpdatePayload {
                business_unit: None,
                name: Some(uuid::Uuid::new_v4().to_string()),
            },
        )
        .await;
    assert_status(&response, 401);
}
