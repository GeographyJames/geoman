use crate::common::{TestApp, helpers::check_landing_page_response};

#[actix_web::test]
async fn get_landing_page_works() {
    let app = TestApp::spawn().await;
    let response = app.ogc_service.get_landing_page(&app.api_client).await;
    check_landing_page_response(response).await;
}
