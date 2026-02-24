use app::handlers::api::teams::TeamInputPayload;
use domain::TeamId;

use crate::common::{AppBuilder, Auth, helpers::assert_status};

#[tokio::test]
async fn post_team_works() {
    let app = AppBuilder::new().build().await;
    let auth = Auth::MockUserCredentials(app.generate_user(true, TeamId(0)).await);
    let bu_id = app.generate_bu_id(Some(&auth)).await;
    let _team_id = app.generate_team_id(Some(&auth), bu_id).await;
}

#[tokio::test]
async fn post_team_requires_admin_permission() {
    let app = AppBuilder::new().build().await;
    let admin_user = app.generate_user(true, TeamId(0)).await;
    let non_admin_user = app.generate_user(false, TeamId(0)).await;
    let bu_id = app
        .generate_bu_id(Some(&Auth::MockUserCredentials(admin_user)))
        .await;

    let response = app
        .teams_service
        .post_json(
            &app.api_client,
            Some(&Auth::MockUserCredentials(non_admin_user)),
            &TeamInputPayload {
                name: uuid::Uuid::new_v4().to_string(),
                business_unit: bu_id,
            },
        )
        .await;
    assert_status(&response, 401);
}
