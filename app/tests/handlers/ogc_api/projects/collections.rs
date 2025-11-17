use crate::common::{
    TestApp,
    helpers::{assert_ok, handle_json_response},
};
use app::enums::ProjectIdentifier;

#[actix_web::test]
async fn get_collections_works_for_project() {
    let app = TestApp::spawn_with_db().await;
    let (_, user_id, project_id) = app.generate_ids().await;
    let (_, collection_id) = app.generate_collection_slug_and_id(user_id).await;
    let _item = app
        .generate_feature_id(collection_id, project_id, user_id, Some({}))
        .await;
    let response = app
        .ogc_service
        .get_project_collections(&app.api_client, &ProjectIdentifier::Id(project_id))
        .await;
    assert_ok(&response);
    let collections: ogc::Collections = handle_json_response(response)
        .await
        .expect("failed to extract ogc collections");
    assert_eq!(collections.collections.len(), 1);
}

#[actix_web::test]
async fn get_collections_only_returns_collectinos_that_contain_items_for_the_project() {
    let app = TestApp::spawn_with_db().await;
    let (_, user_id, project_id) = app.generate_ids().await;
    let another_project = app.generate_project_id(user_id).await;
    let (_, collection_id) = app.generate_collection_slug_and_id(user_id).await;
    let _item = app
        .generate_feature_id(collection_id, another_project, user_id, Some({}))
        .await;
    let response = app
        .ogc_service
        .get_project_collections(&app.api_client, &ProjectIdentifier::Id(project_id))
        .await;
    let ogc_collections: ogc::Collections = handle_json_response(response)
        .await
        .expect("failed to extract ogc collections");
    assert!(ogc_collections.collections.is_empty())
}
