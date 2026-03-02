use domain::TeamId;

use crate::common::{AppBuilder, Auth};

#[tokio::test]
async fn post_business_unit_works() {
    let app = AppBuilder::new().build().await;
    let user = app.generate_user(true, TeamId(0)).await;
    let _bu = app
        .generate_bu_id(Some(&Auth::MockUserCredentials(user)))
        .await;
}
