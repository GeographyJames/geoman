use domain::{Project, Slug};
use ogc::types::FeatureCollection;

use crate::common::{
    TestApp,
    helpers::{assert_ok, handle_json_response},
};

#[actix_web::test]
async fn get_projects_works() {
    let app = TestApp::spawn_with_db().await;
    let response = app
        .ogc_service
        .get_features(
            &app.api_client,
            &Slug::parse("projects".to_string()).unwrap(),
            None,
        )
        .await;
    assert_ok(&response);
    let _projects: FeatureCollection = handle_json_response(response)
        .await
        .expect("failed to retrieve projects");
}
