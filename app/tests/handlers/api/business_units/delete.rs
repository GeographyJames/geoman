use domain::TeamId;

use crate::common::{AppBuilder, Auth, helpers::assert_status};

#[actix_web::test]
async fn delete_business_unit_works() {
    let app = AppBuilder::new().build().await;
    let admin_user = Auth::MockUserCredentials(app.generate_user(true, TeamId(0)).await);
    let bu_id = app.generate_bu_id(Some(&admin_user)).await;
    let response = app
        .business_units_service
        .delete(&app.api_client, bu_id.0, Some(&admin_user))
        .await;
    assert_status(&response, 204);
}
