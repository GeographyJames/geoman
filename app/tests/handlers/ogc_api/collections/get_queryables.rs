use domain::enums::CollectionId;
use ogc::features::filtering::Queryables;

use crate::common::{
    TestApp,
    helpers::{assert_ok, handle_json_response},
};

#[actix_web::test]
async fn get_projects_queryables_works() {
    let app = TestApp::spawn_with_db().await;
    let response = app
        .ogc_service
        .get_collection_queryables(&app.api_client, CollectionId::Projects, None)
        .await;
    assert_ok(&response);
    let _queryables: Queryables = handle_json_response(response)
        .await
        .expect("failed to deserailize json");
}
