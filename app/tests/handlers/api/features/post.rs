use app::handlers::api::project_collections::CollectionReqPayload;
use domain::{FeatureId, ProjectCollectionId, enums::GeometryType};
use gdal::{
    vector::{LayerAccess, OGRwkbGeometryType},
    vsi::get_vsi_mem_file_bytes_owned,
};

use crate::common::{
    AppBuilder, Auth,
    helpers::{
        add_layer, add_shapefile_to_form, add_shz_to_form, assert_ok,
        create_gdal_multipolygon_bng, create_shapefile_dataset, dataset_to_shapefile_data,
        handle_json_response,
    },
};

#[actix_web::test]
async fn post_shapefile_works() {
    let app = AppBuilder::new().build().await;
    let auth = Auth::mock_session_token();
    let collection = CollectionReqPayload {
        title: uuid::Uuid::new_v4().to_string(),
        geometry_type: GeometryType::MultiPolygon,
        description: None,
    };
    let collection_id: ProjectCollectionId = handle_json_response(
        app.collections_service
            .post_json(&app.api_client, Some(&auth), &collection)
            .await,
    )
    .await
    .unwrap();
    let project_id = app.generate_project_id(Some(&auth)).await;
    let mut form = reqwest::multipart::Form::new();
    let input_geom = create_gdal_multipolygon_bng();
    let (mut dataset, filename) = create_shapefile_dataset();
    let mut layer = add_layer(&mut dataset, OGRwkbGeometryType::wkbMultiPolygon, 27700);
    layer
        .create_feature(input_geom.clone())
        .expect("failed to add geom");
    let shapefile_data = dataset_to_shapefile_data(dataset, &filename);
    form = add_shapefile_to_form("test", shapefile_data, form)
        .text("name", uuid::Uuid::new_v4().to_string());
    let response = app
        .features_service
        .post_form(
            &app.api_client,
            form,
            format!("{}/{}", project_id, collection_id,),
            Some(&auth),
        )
        .await;
    assert_ok(&response);
    let feature_id: FeatureId = handle_json_response(response).await.unwrap();
    let response = app
        .ogc_service
        .get_project_feature(&app.api_client, project_id, collection_id, feature_id)
        .await;
    assert_ok(&response);
    let _ogc_ft: ogc::features::Feature = handle_json_response(response).await.unwrap();
}

#[actix_web::test]
async fn post_shapefile_works_with_shz() {
    let app = AppBuilder::new().build().await;
    let auth = Auth::mock_session_token();
    let collection = CollectionReqPayload {
        title: uuid::Uuid::new_v4().to_string(),
        geometry_type: GeometryType::MultiPolygon,
        description: None,
    };
    let collection_id: ProjectCollectionId = handle_json_response(
        app.collections_service
            .post_json(&app.api_client, Some(&auth), &collection)
            .await,
    )
    .await
    .unwrap();
    let project_id = app.generate_project_id(Some(&auth)).await;
    let input_geom = create_gdal_multipolygon_bng();
    let shz_filename = format!("/vsimem/{}.shz", uuid::Uuid::new_v4());
    let driver = gdal::DriverManager::get_driver_by_name("ESRI Shapefile")
        .expect("failed to get shapefile driver");
    let mut dataset = driver
        .create_vector_only(&shz_filename)
        .expect("failed to create shz dataset");
    let mut layer = add_layer(&mut dataset, OGRwkbGeometryType::wkbMultiPolygon, 27700);
    layer
        .create_feature(input_geom.clone())
        .expect("failed to add geom");
    dataset.flush_cache().expect("failed to flush cache");
    dataset.close().expect("failed to close dataset");
    let shz_bytes =
        get_vsi_mem_file_bytes_owned(&shz_filename).expect("failed to read shz bytes");
    let form = add_shz_to_form(shz_bytes, reqwest::multipart::Form::new())
        .text("name", uuid::Uuid::new_v4().to_string());
    let response = app
        .features_service
        .post_form(
            &app.api_client,
            form,
            format!("{}/{}", project_id, collection_id),
            Some(&auth),
        )
        .await;
    assert_ok(&response);
    let feature_id: FeatureId = handle_json_response(response).await.unwrap();
    let response = app
        .ogc_service
        .get_project_feature(&app.api_client, project_id, collection_id, feature_id)
        .await;
    assert_ok(&response);
    let _ogc_ft: ogc::features::Feature = handle_json_response(response).await.unwrap();
}
