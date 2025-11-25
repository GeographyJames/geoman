use crate::common::{
    TestApp,
    helpers::{assert_ok, handle_json_response},
};
use domain::{TableName, enums::CollectionId};

#[actix_web::test]
async fn get_collection_works() {
    let app = TestApp::spawn_with_db().await;
    let collection_id = app.generate_gis_data_table_name().await;

    let response = app
        .ogc_service
        .get_collection(&app.api_client, collection_id.as_ref())
        .await;

    assert_ok(&response);

    let collection: ogcapi_types::common::Collection = handle_json_response(response)
        .await
        .expect("failed to retrieve collection");

    assert_eq!(collection.id, collection_id.as_ref());
    assert!(!collection.links.is_empty());

    let table_one_name = TableName::parse("one".to_string()).unwrap();
    app.create_gis_data_table(
        &table_one_name,
        &domain::enums::GeometryType::Point,
        27700,
        None,
    )
    .await;
    let table_two_name = TableName::parse("two".to_string()).unwrap();
    app.create_gis_data_table(
        &table_two_name,
        &domain::enums::GeometryType::Point,
        27700,
        None,
    )
    .await;

    let response = app
        .ogc_service
        .get_collection(&app.api_client, table_two_name.as_ref())
        .await;
    let collection: ogcapi_types::common::Collection = handle_json_response(response)
        .await
        .expect("failet to retrieve collection");
    assert_eq!(collection.id, table_two_name.as_ref())
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
