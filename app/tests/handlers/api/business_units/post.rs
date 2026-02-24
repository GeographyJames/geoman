use app::handlers::api::business_units::BusinessUnitInputPayload;
use domain::TeamId;

use crate::common::{AppBuilder, Auth, helpers::assert_status};

#[tokio::test]
async fn post_business_unit_works() {
    let app = AppBuilder::new().build().await;
    let user = app.generate_user(true, TeamId(0)).await;
    let _bu = app
        .generate_bu_id(Some(&Auth::MockUserCredentials(user)))
        .await;
}

#[tokio::test]
async fn post_business_unit_requires_admin_permission() {
    let app = AppBuilder::new().build().await;
    let user = app.generate_user(false, TeamId(0)).await;
    let response = app
        .business_units_service
        .post_json(
            &app.api_client,
            Some(&Auth::MockUserCredentials(user)),
            &BusinessUnitInputPayload {
                name: uuid::Uuid::new_v4().to_string(),
            },
        )
        .await;
    assert_status(&response, 401);
}
