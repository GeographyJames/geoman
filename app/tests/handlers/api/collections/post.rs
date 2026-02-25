use app::handlers::api::project_collections::CollectionReqPayload;
use domain::{ProjectCollectionId, TeamId};

use crate::common::{
    AppBuilder, Auth,
    helpers::{assert_ok, assert_status, check_error_response, handle_json_response},
};

#[actix_web::test]
async fn post_collection_works() {
    let app = AppBuilder::new().build().await;
    let _collection_id = app.generate_project_collection_id().await;
}

#[actix_web::test]
async fn post_collection_returns_409_for_duplicate_name() {
    let app = AppBuilder::new().build().await;
    let admin = app.generate_user(true, domain::TeamId(0)).await;
    let collection = CollectionReqPayload::default();
    let _ = app
        .collections_service
        .post_json(
            &app.api_client,
            Some(&Auth::MockUserCredentials(admin.clone())),
            &collection,
        )
        .await;

    let response = app
        .collections_service
        .post_json(
            &app.api_client,
            Some(&Auth::MockUserCredentials(admin)),
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

#[actix_web::test]
async fn only_admins_can_create_global_collections() {
    let app = AppBuilder::new().build().await;
    let collection = CollectionReqPayload::default();
    let response = app
        .collections_service
        .post_json(
            &app.api_client,
            Some(&Auth::mock_session_token()),
            &collection,
        )
        .await;
    assert_status(&response, 401);
}

#[actix_web::test]
async fn non_admin_users_can_create_project_specific_collections() {
    let app = AppBuilder::new().build().await;
    let user = Auth::MockUserCredentials(app.generate_user(false, TeamId(0)).await);
    let project_id = app.generate_project_id(Some(&user)).await;
    let mut collection = CollectionReqPayload::default();
    collection.project_id = Some(project_id);
    let response = app
        .collections_service
        .post_json(&app.api_client, Some(&user), &collection)
        .await;
    assert_ok(&response);
    let collection_id: ProjectCollectionId = handle_json_response(response)
        .await
        .expect("failed to retrieve collection id");
    let _collection = app
        .ogc_service
        .get_project_collection_ogc(&app.api_client, project_id, collection_id)
        .await;
}
