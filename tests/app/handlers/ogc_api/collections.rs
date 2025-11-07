use geoman::ogc::types::common::{Collection, Collections};

use crate::app::{
    TestApp,
    helpers::{assert_ok, handle_json_response},
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
async fn get_collection_works() {
    let app = TestApp::spawn_with_db().await;
    let team_id = app.generate_team_id().await;
    let user_id = app.generate_user_id(team_id).await;
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

    assert_eq!(response.status(), 404);
}
