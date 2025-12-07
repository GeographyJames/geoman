use crate::common::{
    TestApp,
    helpers::{assert_ok, generate_random_wgs84_point_ewkt, handle_json_response},
};
use domain::enums::CollectionId;

#[actix_web::test]
async fn get_collection_works() {
    let app = TestApp::spawn_with_db().await;
    // Create a couple of tables
    let table_one_name = app.generate_gis_data_table_name().await;
    let table_two_name = app.generate_gis_data_table_name().await;

    // Assert the tables have different names
    assert_ne!(table_one_name, table_two_name);

    // Check the response from table two
    let collection: ogcapi_types::common::Collection = handle_json_response(
        app.ogc_service
            .get_collection(&app.api_client, table_two_name.as_ref())
            .await,
    )
    .await
    .expect("failed to retrieve collection");

    // Assert we get the correct collection back
    assert_eq!(collection.id, table_two_name.as_ref());
    assert!(!collection.links.is_empty());
    assert!(
        collection.extent.is_none(),
        "extent should be none beacause there are no features"
    );

    // Add some data to table two
    for _ in 0..10 {
        let (_, _, ewkt) = generate_random_wgs84_point_ewkt();
        app.insert_feature(&table_two_name, &ewkt, &uuid::Uuid::new_v4().to_string())
            .await;
    }

    // Check the response from table two again
    let collection: ogcapi_types::common::Collection = handle_json_response(
        app.ogc_service
            .get_collection(&app.api_client, table_two_name.as_ref())
            .await,
    )
    .await
    .expect("failed to retrieve collection");

    assert!(
        collection.extent.is_some(),
        "extent should be some because we have added features"
    );
}

#[actix_web::test]
async fn get_project_collection_works() {
    let app = TestApp::spawn_with_db().await;
    let response = app
        .ogc_service
        .get_collection(&app.api_client, &CollectionId::Projects.to_string())
        .await;
    assert_ok(&response);
}

#[actix_web::test]
async fn get_collection_includes_projects() {
    let app = TestApp::spawn_with_db().await;
    let response = app.ogc_service.get_collections(&app.api_client).await;
    assert_ok(&response);
    let collections: ogcapi_types::common::Collections = handle_json_response(response)
        .await
        .expect("failed to retrieve collections");
    let _projects_collection = collections
        .collections
        .iter()
        .find(|c| c.id == CollectionId::Projects.to_string())
        .expect("no projects collection");
}
