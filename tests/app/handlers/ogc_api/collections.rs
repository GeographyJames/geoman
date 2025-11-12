use geoman::{
    constants::DB_QUERY_FAIL,
    ogc::types::common::{Collection, Collections},
};

use crate::app::{
    TestApp,
    helpers::{assert_ok, assert_status, check_error_response, handle_json_response},
};

#[actix_web::test]
async fn get_collections_works() {
    let app = TestApp::spawn_with_db().await;

    let response = app.ogc_service.get_collections(&app.api_client).await;

    assert_ok(&response);

    let collections: Collections = handle_json_response(response)
        .await
        .expect("failed to retrieve collections");

    assert_eq!(collections.collections.len(), 0)
}

#[actix_web::test]
async fn get_collections_returns_500_when_db_corrupt() {
    let app = TestApp::spawn_with_db().await;
    app.drop_app_schema().await;
    let response = app.ogc_service.get_collections(&app.api_client).await;
    check_error_response(response, 500, DB_QUERY_FAIL).await;
}

#[actix_web::test]
async fn get_collection_works() {
    let app = TestApp::spawn_with_db().await;
    let (_, user_id, _) = app.generate_ids().await;
    let (collection_slug, _) = app.generate_collection_slug_and_id(user_id).await;

    let response = app
        .ogc_service
        .get_collection(&app.api_client, collection_slug.as_ref())
        .await;

    assert_ok(&response);

    let collection: Collection = handle_json_response(response)
        .await
        .expect("failed to retrieve collection");

    assert_eq!(&collection.id, collection_slug.as_ref());
    assert!(!collection.links.is_empty());
}

#[actix_web::test]
async fn get_collection_returns_404_when_not_found() {
    let app = TestApp::spawn_with_db().await;

    let response = app
        .ogc_service
        .get_collection(&app.api_client, &uuid::Uuid::new_v4().to_string())
        .await;

    assert_status(&response, 404);
}

#[actix_web::test]
async fn get_collection_returns_500_when_db_corrupt() {
    let app = TestApp::spawn_with_db().await;
    app.drop_app_schema().await;
    let response = app
        .ogc_service
        .get_collection(&app.api_client, &uuid::Uuid::new_v4().to_string())
        .await;
    check_error_response(response, 500, DB_QUERY_FAIL).await;
}
