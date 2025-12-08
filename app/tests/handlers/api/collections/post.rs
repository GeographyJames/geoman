use app::{
    constants::SITE_BOUNDARIES_COLLECTION_NAME,
    handlers::api::project_collections::CollectionReqPayload,
};

use crate::common::{AppBuilder, Auth, helpers::check_error_response};

#[actix_web::test]
async fn post_collection_works() {
    let app = AppBuilder::new().build().await;
    let _collection_id = app
        .generate_project_collection_id(Some(&Auth::mock_session_token()))
        .await;
}

#[actix_web::test]
async fn post_collection_returns_409_for_duplicate_name() {
    let app = AppBuilder::new().build().await;
    let mut collection = CollectionReqPayload::default();
    collection.title = SITE_BOUNDARIES_COLLECTION_NAME.to_string();
    let response = app
        .collections_service
        .post_json(
            &app.api_client,
            Some(&Auth::mock_session_token()),
            &collection,
        )
        .await;
    let err = check_error_response(response, 409).await;
    assert!(
        err.message
            .to_lowercase()
            .contains("a collection with this name")
    )
}
