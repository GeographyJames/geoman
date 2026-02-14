use crate::common::{Auth, TestApp, helpers::assert_ok};

#[actix_web::test]
async fn get_project_openapi_works() {
    let app = TestApp::spawn_with_db().await;
    let project_id = app
        .generate_project_id(Some(&Auth::mock_session_token()))
        .await;
    let response = app
        .ogc_service
        .get_project_openapi(&app.api_client, project_id)
        .await;
    assert_ok(&response);
}
