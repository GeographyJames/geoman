use domain::ProjectId;

use crate::common::{
    TestApp,
    helpers::{assert_ok, assert_status, handle_json_response},
};

#[actix_web::test]
async fn get_collections_works_for_project() {
    let app = TestApp::spawn_with_db().await;
    let (_, user_id, project_id) = app.generate_ids().await;
    let collection_id = app.generate_collection_id(user_id).await;
    let _item = app
        .generate_feature_id(collection_id, project_id, user_id, Some({}))
        .await;
    let response = app
        .ogc_service
        .get_project_collections(&app.api_client, project_id)
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
    let collection_id = app.generate_collection_id(user_id).await;
    let _item = app
        .generate_feature_id(collection_id, another_project, user_id, Some({}))
        .await;
    let response = app
        .ogc_service
        .get_project_collections(&app.api_client, project_id)
        .await;
    let ogc_collections: ogc::Collections = handle_json_response(response)
        .await
        .expect("failed to extract ogc collections");
    assert!(ogc_collections.collections.is_empty())
}

#[actix_web::test]
async fn get_project_collection_works() {
    let app = TestApp::spawn_with_db().await;
    let (_, user_id, project_id) = app.generate_ids().await;
    let collection_id = app.generate_collection_id(user_id).await;
    let _feature_id = app
        .generate_feature_id(collection_id, project_id, user_id, Some({}))
        .await;
    let response = app
        .ogc_service
        .get_project_collection(&app.api_client, project_id, collection_id)
        .await;
    assert_ok(&response);
}

#[actix_web::test]
async fn get_project_collection_returns_404_for_project_not_found() {
    let app = TestApp::spawn_with_db().await;
    let (_, user_id, _) = app.generate_ids().await;
    let collection_id = app.generate_collection_id(user_id).await;
    let response = app
        .ogc_service
        .get_project_collection(&app.api_client, ProjectId::default(), collection_id)
        .await;
    assert_status(&response, 404);
}

#[actix_web::test]
async fn get_project_collection_returns_404_for_collection_with_no_features() {
    let app = TestApp::spawn_with_db().await;
    let (_, user_id, project_id) = app.generate_ids().await;
    let another_project = app.generate_project_id(user_id).await;
    let collection_id = app.generate_collection_id(user_id).await;
    let _feature = app
        .generate_feature_id(collection_id, another_project, user_id, Some({}))
        .await;
    let response = app
        .ogc_service
        .get_project_collection(&app.api_client, project_id, collection_id)
        .await;
    assert_status(&response, 404);
}
