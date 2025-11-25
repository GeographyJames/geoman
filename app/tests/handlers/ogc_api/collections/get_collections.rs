use crate::common::{
    TestApp,
    helpers::{assert_ok, handle_json_response},
};

#[actix_web::test]
async fn get_collections_works() {
    let app = TestApp::spawn_with_db().await;
    let _ = app.generate_gis_data_table_name().await;
    let response = app.ogc_service.get_collections(&app.api_client).await;

    assert_ok(&response);
    let collections: ogcapi_types::common::Collections = handle_json_response(response)
        .await
        .expect("failed to retrieve collections");
    assert_eq!(collections.collections.len(), 2);
}
