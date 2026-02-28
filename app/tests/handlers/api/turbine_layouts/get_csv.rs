use domain::TeamId;

use crate::common::{AppBuilder, Auth};

#[tokio::test]
async fn get_csv_works() {
    let app = AppBuilder::new().build().await;
    let user = Auth::MockUserCredentials(app.generate_user(false, TeamId(0)).await);
    let project_id = app.generate_project_id(Some(&user)).await;
    let _layout_id = app
        .generate_primary_layout_id(&project_id, Some(&user))
        .await;
}
