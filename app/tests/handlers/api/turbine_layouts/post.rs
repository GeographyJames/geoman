use domain::TeamId;

use crate::common::{AppBuilder, Auth, helpers::assert_ok};

#[tokio::test]
async fn post_turbine_layout_works() {
    let app = AppBuilder::new().build().await;
    let auth = Auth::MockUserCredentials(app.generate_user(false, TeamId(0)).await);
    let project_id = app.generate_project_id(Some(&auth)).await;
    let response = app.generate_primary_layout(&project_id, Some(&auth)).await;
    assert_ok(&response)
}
