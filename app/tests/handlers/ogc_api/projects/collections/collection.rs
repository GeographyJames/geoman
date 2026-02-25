use domain::ProjectId;

use crate::common::{
    Auth, TestApp,
    helpers::{assert_ok, assert_status, create_gdal_point_bng, create_gdal_point_wgs84},
};

#[actix_web::test]
async fn get_project_collection_has_correct_storage_crs() {
    let app = TestApp::spawn_with_db().await;
    let auth = Auth::mock_session_token();
    let project_one_id = app.generate_project_id(Some(&auth)).await;
    let project_two_id = app.generate_project_id(Some(&auth)).await;
    let collection_id = app.generate_project_collection_id().await;
    let bng = create_gdal_point_bng();
    let wgs84 = create_gdal_point_wgs84();

    // Insert a feature for project one
    let _ = app
        .insert_project_feature(collection_id, project_one_id, bng, 27700, Some(&auth), None)
        .await;

    // Retrieve the collection
    let collection = app
        .ogc_service
        .get_project_collection_ogc(&app.api_client, project_one_id, collection_id)
        .await;

    // Assert the colection has the correct srid
    assert_eq!(
        collection.storage_crs.expect("no storage crs").as_srid(),
        27700
    );

    // Insert a second feature in a different crs to another project
    let _ = app
        .insert_project_feature(
            collection_id,
            project_two_id,
            wgs84.clone(),
            4326,
            Some(&auth),
            None,
        )
        .await;

    // Retrieve the colection for project one again
    let collection = app
        .ogc_service
        .get_project_collection_ogc(&app.api_client, project_one_id, collection_id)
        .await;

    // It should still have a storage crs because they are for different projects
    assert_eq!(
        collection.storage_crs.expect("no storage crs").as_srid(),
        27700
    );

    // Retrieve the collection for project two
    let collection = app
        .ogc_service
        .get_project_collection_ogc(&app.api_client, project_two_id, collection_id)
        .await;

    // The storage crs should be reported
    assert_eq!(
        collection.storage_crs.expect("no strorage crs").as_srid(),
        4326
    );

    // Finally lets add a feature in anothe crs to for project one
    let _ = app
        .insert_project_feature(
            collection_id,
            project_one_id,
            wgs84,
            4326,
            Some(&auth),
            None,
        )
        .await;
    let collection = app
        .ogc_service
        .get_project_collection_ogc(&app.api_client, project_one_id, collection_id.into())
        .await;

    // Now there should be no storage crs
    assert!(collection.storage_crs.is_none(),);
}

#[actix_web::test]
async fn get_project_collection_works() {
    let app = TestApp::spawn_with_db().await;
    let auth = Auth::mock_session_token();
    let project_id = app.generate_project_id(Some(&auth)).await;
    let collection_id = app.generate_project_collection_id().await;
    let _feature_id = app
        .generate_project_feature_id(collection_id, project_id, Some(&auth))
        .await;
    let response = app
        .ogc_service
        .get_project_collection(&app.api_client, project_id, collection_id)
        .await;
    assert_ok(&response);
}

#[actix_web::test]
async fn get_project_collection_returns_404_for_project_not_found() {
    let app = TestApp::spawn_with_db().await;
    let collection_id = app.generate_project_collection_id().await;
    let response = app
        .ogc_service
        .get_project_collection(&app.api_client, ProjectId::default(), collection_id)
        .await;
    assert_status(&response, 404);
}

#[actix_web::test]
async fn get_project_collection_returns_404_for_collection_with_no_features() {
    let app = TestApp::spawn_with_db().await;
    let auth = Auth::mock_session_token();
    let project_id = app.generate_project_id(Some(&auth)).await;
    let another_project = app.generate_project_id(Some(&auth)).await;
    let collection_id = app.generate_project_collection_id().await;
    let _feature = app
        .generate_project_feature_id(collection_id, another_project, Some(&auth))
        .await;
    let response = app
        .ogc_service
        .get_project_collection(&app.api_client, project_id, collection_id)
        .await;
    assert_status(&response, 404);
}

#[actix_web::test]
async fn get_project_collection_has_correct_crs_list() {
    let app = TestApp::spawn_with_db().await;
    let auth = Auth::mock_session_token();
    let project_id = app.generate_project_id(Some(&auth)).await;
    let collection_id = app.generate_project_collection_id().await;
    let _feature_id = app
        .generate_project_feature_id(collection_id, project_id, Some(&auth))
        .await;

    let collection = app
        .ogc_service
        .get_project_collection_ogc(&app.api_client, project_id, collection_id)
        .await;

    assert_eq!(collection.crs.len(), 2);
    app.insert_project_feature(
        collection_id,
        project_id,
        create_gdal_point_wgs84(),
        4326,
        Some(&auth),
        None,
    )
    .await;
    let collection = app
        .ogc_service
        .get_project_collection_ogc(&app.api_client, project_id, collection_id)
        .await;
    assert_eq!(collection.crs.len(), 1)
}
