use crate::common::{TestApp, helpers::assert_ok};

#[actix_web::test]
async fn get_project_openapi_works() {
    let app = TestApp::spawn_with_db().await;
    let (_, _, project_id) = app.generate_ids().await;
    let response = app
        .ogc_service
        .get_project_openapi(&app.api_client, project_id)
        .await;
    assert_ok(&response);
}
