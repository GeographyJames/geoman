use app::handlers::api::project_collections::{CollectionReqPayload, PatchCollectionPayload};
use domain::{CollectionListItem, ProjectCollectionId, TeamId};

use crate::common::{
    AppBuilder, Auth, TestApp,
    helpers::{assert_status, handle_json_response},
};

#[actix_web::test]
pub async fn patch_collection_works() {
    let app = AppBuilder::new().build().await;
    let auth = Auth::MockUserCredentials(app.generate_user(true, TeamId(0)).await);

    let collection_id = app.generate_project_collection_id().await;

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

#[actix_web::test]
pub async fn only_admin_can_patch_global_collections() {
    let app = AppBuilder::new().build().await;
    let collection_id = app.generate_project_collection_id().await;
    let auth = Auth::MockUserCredentials(app.generate_user(false, TeamId(0)).await);
    let update_payload = PatchCollectionPayload::default();
    let response = app
        .collections_service
        .patch_json(&app.api_client, collection_id, Some(&auth), &update_payload)
        .await;
    assert_status(&response, 401);
}

#[actix_web::test]
async fn only_project_team_can_patch_collection() {
    let (app, owner, project_id) = TestApp::with_project().await;
    let admin = Auth::MockUserCredentials(app.generate_user(true, TeamId(0)).await);
    let other_team = app.generate_team_id(Some(&admin)).await;
    let outsider = Auth::MockUserCredentials(app.generate_user(false, other_team).await);
    let collection = CollectionReqPayload {
        project_id: Some(project_id),
        ..CollectionReqPayload::default()
    };
    let collection_id: ProjectCollectionId = handle_json_response(
        app.collections_service
            .post_json(&app.api_client, Some(&owner), &collection)
            .await,
    )
    .await
    .expect("failed to create project collection");
    let response = app
        .collections_service
        .patch_json(
            &app.api_client,
            collection_id,
            Some(&outsider),
            &PatchCollectionPayload::default(),
        )
        .await;
    assert_status(&response, 401);
}
