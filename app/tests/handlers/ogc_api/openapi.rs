use crate::common::{TestApp, helpers::assert_ok};

#[actix_web::test]
async fn get_openapi_works() {
    let app = TestApp::spawn(None).await;
    let response = app.ogc_service.get_openapi(&app.api_client).await;
    assert_ok(&response);
}
