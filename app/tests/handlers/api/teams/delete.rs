use domain::TeamId;

use crate::common::{AppBuilder, Auth, helpers::assert_status};

#[actix_web::test]
async fn delete_team_works() {
    let app = AppBuilder::new().build().await;
    let admin_user = Auth::MockUserCredentials(app.generate_user(true, TeamId(0)).await);
    let team_id = app.generate_team_id(Some(&admin_user)).await;
    let response = app
        .teams_service
        .delete(&app.api_client, team_id.0, Some(&admin_user))
        .await;
    assert_status(&response, 204);
}

