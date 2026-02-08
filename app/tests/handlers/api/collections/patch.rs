use app::handlers::api::project_collections::{CollectionReqPayload, PatchCollectionPayload};
use domain::{CollectionListItem, ProjectCollectionId};

use crate::common::{
    AppBuilder, Auth,
    helpers::{assert_status, handle_json_response},
};

#[actix_web::test]
pub async fn patch_collection_works() {
    let app = AppBuilder::new().build().await;
    let auth = Auth::mock_session_token();
    let collection = CollectionReqPayload::default();

    let collection_id: ProjectCollectionId = handle_json_response(
        app.collections_service
            .post_json(&app.api_client, Some(&auth), &collection)
            .await,
    )
    .await
    .unwrap();

    let collections: Vec<CollectionListItem> = handle_json_response(
        app.collections_service
            .get(&app.api_client, Some(&auth))
            .await,
    )
    .await
    .unwrap();
    let saved_collection = collections.iter().find(|c| c.id == collection_id);
    assert_eq!(saved_collection.unwrap().title, collection.title);
    let new_name = uuid::Uuid::new_v4().to_string();
    let mut update_payload = PatchCollectionPayload::default();
    update_payload.title = Some(new_name.clone());
    let response = app
        .collections_service
        .patch_json(&app.api_client, collection_id, Some(&auth), &update_payload)
        .await;
    assert_status(&response, 204);
    let collections: Vec<CollectionListItem> = handle_json_response(
        app.collections_service
            .get(&app.api_client, Some(&auth))
            .await,
    )
    .await
    .unwrap();
    let saved_collection = collections.iter().find(|c| c.id == collection_id);
    assert_eq!(saved_collection.unwrap().title, new_name);
}
