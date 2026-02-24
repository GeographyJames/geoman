use domain::BusinessUnit;

use crate::common::{
    AppBuilder, Auth,
    helpers::{assert_ok, handle_json_response},
};

#[tokio::test]
async fn get_business_units_works() {
    let app = AppBuilder::new().build().await;
    let auth = Auth::mock_session_token();
    let response = app
        .business_units_service
        .get(&app.api_client, Some(&auth))
        .await;
    assert_ok(&response);
    let _business_units: Vec<BusinessUnit> = handle_json_response(response)
        .await
        .expect("failed to retrieve business units");
}
