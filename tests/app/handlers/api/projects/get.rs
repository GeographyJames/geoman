use crate::app::{TestApp, helpers::assert_ok};

#[actix_web::test]
async fn get_projects_works() {
    let app = TestApp::spawn().await;
    let token = app.get_test_session_token().await;
    let response = app
        .projects_service
        .get(&app.api_client, Some(&token))
        .await;
    assert_ok(&response);
}
