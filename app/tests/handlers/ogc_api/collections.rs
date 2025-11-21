use crate::common::{
    TestApp,
    helpers::{
        assert_ok, generate_random_bng_point_ewkt, generate_random_wgs84_point_ewkt,
        handle_json_response,
    },
};
use domain::enums::CollectionId;

#[actix_web::test]
async fn get_collections_works() {
    let app = TestApp::spawn_with_db().await;
    let (_, user_id, _) = app.generate_ids().await;
    let _ = app.generate_collection_id(user_id).await;
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
    let collection_id = app.generate_collection_id(user_id).await;

    let response = app
        .ogc_service
        .get_collection(&app.api_client, collection_id.into())
        .await;

    assert_ok(&response);

    let collection: ogc::Collection = handle_json_response(response)
        .await
        .expect("failed to retrieve collection");

    assert_eq!(collection.id, collection_id.0.to_string());
    assert!(!collection.links.is_empty());
}

#[actix_web::test]
async fn get_project_collection_works() {
    let app = TestApp::spawn_with_db().await;
    let response = app
        .ogc_service
        .get_collection(&app.api_client, CollectionId::Projects)
        .await;
    assert_ok(&response);
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
        .find(|c| c.id == CollectionId::Projects.to_string())
        .expect("no projects collection");
}

#[actix_web::test]
async fn get_collection_has_correct_storage_crs() {
    let app = TestApp::spawn_with_db().await;
    let (_, user_id, project_id) = app.generate_ids().await;
    let another_project = app.generate_project_id(user_id).await;
    let collection_id = app.generate_collection_id(user_id).await;
    let (_, _, bng_ewkt) = generate_random_bng_point_ewkt();
    let (_, _, wges84_ewkt) = generate_random_wgs84_point_ewkt();
    let _ = app
        .insert_feature(
            &uuid::Uuid::new_v4().to_string(),
            collection_id,
            project_id,
            user_id,
            &bng_ewkt,
            Some({}),
        )
        .await;

    let response = app
        .ogc_service
        .get_collection(&app.api_client, collection_id.into())
        .await;
    let collection: ogcapi_types::common::Collection = handle_json_response(response)
        .await
        .expect("failed to retrieve collection");
    assert_eq!(
        collection.storage_crs.expect("no storage crs").as_srid(),
        27700
    );

    let response = app
        .ogc_service
        .get_project_collection(&app.api_client, project_id, collection_id)
        .await;
    let collection: ogcapi_types::common::Collection = handle_json_response(response)
        .await
        .expect("failed to retrieve collection");
    assert_eq!(
        collection.storage_crs.expect("no storage crs").as_srid(),
        27700
    );

    //
    let _ = app
        .insert_feature(
            &uuid::Uuid::new_v4().to_string(),
            collection_id,
            another_project,
            user_id,
            &wges84_ewkt,
            Some({}),
        )
        .await;
    let response = app
        .ogc_service
        .get_collection(&app.api_client, collection_id.into())
        .await;
    let collection: ogcapi_types::common::Collection = handle_json_response(response)
        .await
        .expect("failed to retrieve collection");
    assert!(collection.storage_crs.is_none());
    let response = app
        .ogc_service
        .get_project_collection(&app.api_client, project_id, collection_id.into())
        .await;
    let collection: ogcapi_types::common::Collection = handle_json_response(response)
        .await
        .expect("failed to retrieve collection");
    assert_eq!(
        collection.storage_crs.expect("no storage crs").as_srid(),
        27700
    );
    let _ = app
        .insert_feature(
            &uuid::Uuid::new_v4().to_string(),
            collection_id,
            project_id,
            user_id,
            &wges84_ewkt,
            Some({}),
        )
        .await;
    let response = app
        .ogc_service
        .get_project_collection(&app.api_client, project_id, collection_id.into())
        .await;
    let collection: ogcapi_types::common::Collection = handle_json_response(response)
        .await
        .expect("failed to retrieve collection");
    assert!(collection.storage_crs.is_none(),);
}
