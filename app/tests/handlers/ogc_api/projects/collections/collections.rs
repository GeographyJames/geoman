use app::handlers::api::project_collections::CollectionReqPayload;
use domain::TeamId;

use crate::common::{
    AppBuilder, Auth, TestApp,
    helpers::{assert_ok, handle_json_response},
};

#[actix_web::test]
async fn get_project_collections_works() {
    let app = TestApp::spawn_with_db().await;
    let auth = Auth::mock_session_token();
    let project_id = app.generate_project_id(Some(&auth)).await;
    let collection_id = app.generate_project_collection_id().await;

    let _feature_id = app
        .generate_project_feature_id(collection_id, project_id, Some(&auth))
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
    let project_id = app.generate_project_id(Some(&auth)).await;
    let another_project = app.generate_project_id(Some(&auth)).await;
    let collection_id = app.generate_project_collection_id().await;
    let _feature_id = app
        .generate_project_feature_id(collection_id, another_project, Some(&auth))
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

#[actix_web::test]
async fn get_collections_includes_project_specific_collections() {
    let app = AppBuilder::new().build().await;
    let user = Auth::MockUserCredentials(app.generate_user(false, TeamId(0)).await);
    let project_id = app.generate_project_id(Some(&user)).await;
    let mut collection = CollectionReqPayload::default();
    collection.project_id = Some(project_id);
    let _ = app
        .collections_service
        .post_json(&app.api_client, Some(&user), &collection)
        .await;

    let response = app
        .ogc_service
        .get_project_collections(&app.api_client, project_id)
        .await;
    let ogc_collections: ogcapi_types::common::Collections = handle_json_response(response)
        .await
        .expect("failed to extract ogc collections");
    assert_eq!(ogc_collections.collections.len(), 1)
}

#[actix_web::test]
async fn get_collections_includes_turbine_layouts() {
    let app = AppBuilder::new().build().await;
    let user = Auth::MockUserCredentials(app.generate_user(false, TeamId(0)).await);
    let project_id = app.generate_project_id(Some(&user)).await;
    let _ = app
        .generate_primary_layout_id(&project_id, Some(&user))
        .await;
    let response = app
        .ogc_service
        .get_project_collections(&app.api_client, project_id)
        .await;
    let ogc_collections: ogcapi_types::common::Collections = handle_json_response(response)
        .await
        .expect("failed to extract ogc collections");
    let _turbine_layouts = ogc_collections
        .collections
        .iter()
        .find(|&c| c.title.as_deref() == Some("turbine layouts"))
        .expect("turbine collection not found");
}

#[actix_web::test]
async fn get_collections_does_not_include_turbine_layouts_for_other_projects() {
    let app = AppBuilder::new().build().await;
    let user = Auth::MockUserCredentials(app.generate_user(false, TeamId(0)).await);
    let project_a = app.generate_project_id(Some(&user)).await;
    let project_b = app.generate_project_id(Some(&user)).await;
    let _ = app
        .generate_primary_layout_id(&project_a, Some(&user))
        .await;
    let response = app
        .ogc_service
        .get_project_collections(&app.api_client, project_b)
        .await;
    let ogc_collections: ogcapi_types::common::Collections = handle_json_response(response)
        .await
        .expect("failed to extract ogc collections");
    let has_turbine_layouts = ogc_collections
        .collections
        .iter()
        .any(|c| c.title.as_deref() == Some("turbine layouts"));
    assert!(!has_turbine_layouts);
}
