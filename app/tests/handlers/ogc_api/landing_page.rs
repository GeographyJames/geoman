use ogcapi_types::common::LandingPage;

use crate::common::{
    TestApp,
    helpers::{assert_ok, handle_json_response},
};

#[actix_web::test]
async fn get_landing_page_works() {
    let app = TestApp::spawn(None).await;
    let response = app
        .ogc_service
        .get_landing_page(&app.api_client, None)
        .await;
    assert_ok(&response);
    handle_json_response::<LandingPage>(response)
        .await
        .expect("failed to retrieve landing page");
}
