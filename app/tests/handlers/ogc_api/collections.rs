use app::enums::Collection;

use crate::common::{
    TestApp,
    helpers::{assert_ok, handle_json_response},
};

#[actix_web::test]
async fn get_collections_works() {
    let app = TestApp::spawn_with_db().await;
    let (_, user_id, _) = app.generate_ids().await;
    let _collection = app.generate_collection_slug_and_id(user_id).await;
    let response = app.ogc_service.get_collections(&app.api_client).await;

    assert_ok(&response);
    let collections: ogc::Collections = handle_json_response(response)
        .await
        .expect("failed to retrieve collections");
    assert_eq!(collections.collections.len(), 2);
}

#[actix_web::test]
async fn get_collection_works() {
    let app = TestApp::spawn_with_db().await;
    let (_, user_id, _) = app.generate_ids().await;
    let (slug, _) = app.generate_collection_slug_and_id(user_id).await;

    let response = app.ogc_service.get_collection(&app.api_client, &slug).await;

    assert_ok(&response);

    let collection: ogc::Collection = handle_json_response(response)
        .await
        .expect("failed to retrieve collection");

    assert_eq!(&collection.id, slug.as_ref());
    assert!(!collection.links.is_empty());
}

#[actix_web::test]
async fn get_collection_includes_projects() {
    let app = TestApp::spawn_with_db().await;
    let response = app.ogc_service.get_collections(&app.api_client).await;
    assert_ok(&response);
    let collections: ogc::Collections = handle_json_response(response)
        .await
        .expect("failed to retrieve collections");
    let _projects_collection = collections
        .collections
        .iter()
        .find(|c| c.id == Collection::Projects.to_string())
        .expect("no projects collection");
}
