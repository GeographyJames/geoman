use app::{
    constants::SITE_BOUNDARIES_COLLECTION_NAME,
    handlers::api::project_collections::CollectionReqPayload,
};
use domain::ProjectCollectionId;

use crate::common::{
    AppBuilder,
    helpers::{check_error_response, handle_json_response},
};

#[actix_web::test]
async fn post_collection_works() {
    let app = AppBuilder::new().build().await;
    let collection = CollectionReqPayload::default();
    let response = app
        .collections_service
        .post_json(&app.api_client, None, &collection)
        .await;
    let _id: ProjectCollectionId = handle_json_response(response)
        .await
        .expect("failed to retrieve collection id");
}

#[actix_web::test]
async fn post_collection_returns_409_for_duplicate_name() {
    let app = AppBuilder::new().build().await;
    let mut collection = CollectionReqPayload::default();
    collection.title = SITE_BOUNDARIES_COLLECTION_NAME.to_string();
    let response = app
        .collections_service
        .post_json(&app.api_client, None, &collection)
        .await;
    let err = check_error_response(response, 409).await;
    assert!(
        err.message
            .to_lowercase()
            .contains("a collection with this name")
    )
}
