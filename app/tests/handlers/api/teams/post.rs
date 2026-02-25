use app::handlers::api::teams::TeamInputPayload;
use domain::TeamId;

use crate::common::{AppBuilder, Auth, helpers::assert_status};

#[tokio::test]
async fn post_team_works() {
    let app = AppBuilder::new().build().await;
    let auth = Auth::MockUserCredentials(app.generate_user(true, TeamId(0)).await);

    let _team_id = app.generate_team_id(Some(&auth)).await;
}

#[tokio::test]
async fn post_team_requires_admin_permission() {
    let app = AppBuilder::new().build().await;
    let non_admin_user = app.generate_user(false, TeamId(0)).await;

    let response = app
        .teams_service
        .post_json(
            &app.api_client,
            Some(&Auth::MockUserCredentials(non_admin_user)),
            &TeamInputPayload {
                name: uuid::Uuid::new_v4().to_string(),
                business_unit: None,
            },
        )
        .await;
    assert_status(&response, 401);
}
