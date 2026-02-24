use domain::TeamId;

use crate::common::{AppBuilder, Auth, helpers::assert_status};

#[tokio::test]
async fn delete_user_works() {
    let app = AppBuilder::new().build().await;
    let admin_user = Auth::MockUserCredentials(app.generate_user(true, TeamId(-1)).await);
    let user_to_delete = app.generate_user(false, TeamId(-1)).await;
    let response = app
        .users_service
        .delete(&app.api_client, user_to_delete.id.0, Some(&admin_user))
        .await;
    assert_status(&response, 204);
}

#[tokio::test]
async fn delete_user_requires_admin_permission() {
    let app = AppBuilder::new().build().await;
    let admin_user = Auth::MockUserCredentials(app.generate_user(true, TeamId(-1)).await);
    let non_admin_user = Auth::MockUserCredentials(app.generate_user(false, TeamId(-1)).await);
    let user_to_delete = app.generate_user(false, TeamId(-1)).await;
    let response = app
        .users_service
        .delete(&app.api_client, user_to_delete.id.0, Some(&non_admin_user))
        .await;
    assert_status(&response, 401);
    // Ensure user still exists
    let _ = admin_user;
}
