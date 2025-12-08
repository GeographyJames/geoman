use crate::common::{
    Auth, TestApp,
    helpers::{assert_ok, handle_json_response},
};

#[actix_web::test]
async fn get_project_collections_works() {
    let app = TestApp::spawn_with_db().await;
    let auth = Auth::mock_session_token();
    let (_, user_id, project_id) = app.generate_ids().await;
    let collection_id = app.generate_project_collection_id(Some(&auth)).await;
    let _item = app
        .generate_project_feature_id(collection_id, project_id, user_id, Some({}))
        .await;
    let response = app
        .ogc_service
        .get_project_collections(&app.api_client, project_id)
        .await;
    assert_ok(&response);
    let collections: ogcapi_types::common::Collections = handle_json_response(response)
        .await
        .expect("failed to extract ogc collections");
    assert_eq!(collections.collections.len(), 1);
}

#[actix_web::test]
async fn get_project_collections_only_returns_collections_that_contain_items_for_the_project() {
    let app = TestApp::spawn_with_db().await;
    let auth = Auth::mock_session_token();
    let (_, user_id, project_id) = app.generate_ids().await;
    let another_project = app
        .generate_project_id(Some(&auth))
        .await;
    let collection_id = app.generate_project_collection_id(Some(&auth)).await;
    let _item = app
        .generate_project_feature_id(collection_id, another_project, user_id, Some({}))
        .await;
    let response = app
        .ogc_service
        .get_project_collections(&app.api_client, project_id)
        .await;
    let ogc_collections: ogcapi_types::common::Collections = handle_json_response(response)
        .await
        .expect("failed to extract ogc collections");
    assert!(ogc_collections.collections.is_empty())
}
