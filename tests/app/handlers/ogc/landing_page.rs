use geoman::ogc::types::common::LandingPage;

use crate::app::{TestApp, helpers::handle_json_response};

#[actix_web::test]
async fn get_landing_page_works() {
    let app = TestApp::spawn().await;

    let response = app.ogc_service.get_landing_page(&app.api_client).await;

    let _landing_page: LandingPage = handle_json_response(response)
        .await
        .expect("failed to retrieve landing page");
}
