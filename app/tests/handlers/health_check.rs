use crate::common::{TestApp, helpers::assert_ok};

#[actix_web::test]
async fn health_check_works() {
    let app = TestApp::spawn(None).await;
    let response = app.health_check_service.get(&app.api_client, None).await;
    assert_ok(&response)
}
