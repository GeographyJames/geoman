use crate::common::{
    AppBuilder, Auth,
    helpers::{add_layer, assert_ok, create_gdal_multipolygon_bng, handle_json_response},
};
use gdal::vector::{LayerAccess, OGRwkbGeometryType};
use gdal::vsi::get_vsi_mem_file_bytes_owned;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct EpsgRequest {
    prj: String,
}

#[derive(Deserialize, Debug)]
struct EpsgResponse {
    srid: i32,
}

#[actix_web::test]
pub async fn returns_epsg_code_for_esri_style_prj() {
    let app = AppBuilder::new().build().await;
    let auth = Auth::mock_session_token();
    let prj = std::fs::read_to_string("../test-data/shapefiles/3_valid_polygon_osgb36.prj")
        .expect("failed to read .prj file");
    let body = EpsgRequest { prj };
    let response = app
        .epsg_service
        .post_json(&app.api_client, Some(&auth), &body)
        .await;
    assert_ok(&response);
    let result: EpsgResponse = handle_json_response(response).await.unwrap();
    assert_eq!(result.srid, 27700);
}

#[actix_web::test]
pub async fn returns_epsg_code_for_standard_wkt() {
    let app = AppBuilder::new().build().await;
    let auth = Auth::mock_session_token();
    let srs = gdal::spatial_ref::SpatialRef::from_epsg(4326).expect("failed to create spatial ref");
    let prj = srs.to_wkt().expect("failed to convert to WKT");
    let body = EpsgRequest { prj };
    let response = app
        .epsg_service
        .post_json(&app.api_client, Some(&auth), &body)
        .await;
    assert_ok(&response);
    let result: EpsgResponse = handle_json_response(response).await.unwrap();
    assert_eq!(result.srid, 4326);
}

#[actix_web::test]
pub async fn returns_epsg_code_from_shz() {
    let app = AppBuilder::new().build().await;
    let auth = Auth::mock_session_token();
    let geom = create_gdal_multipolygon_bng();
    let shz_filename = format!("/vsimem/{}.shz", uuid::Uuid::new_v4());
    let driver = gdal::DriverManager::get_driver_by_name("ESRI Shapefile")
        .expect("failed to get shapefile driver");
    let mut dataset = driver
        .create_vector_only(&shz_filename)
        .expect("failed to create shz dataset");
    let mut layer = add_layer(&mut dataset, OGRwkbGeometryType::wkbMultiPolygon, 27700);
    layer.create_feature(geom).expect("failed to add geom");
    dataset.flush_cache().expect("failed to flush cache");
    dataset.close().expect("failed to close dataset");
    let shz_bytes = get_vsi_mem_file_bytes_owned(&shz_filename).expect("failed to read shz bytes");
    let form = reqwest::multipart::Form::new().part(
        "shz",
        reqwest::multipart::Part::bytes(shz_bytes)
            .file_name("test.shz")
            .mime_str("application/octet-stream")
            .expect("failed to create part"),
    );
    let response = app
        .epsg_service
        .post_form(&app.api_client, form, "shz", Some(&auth))
        .await;
    assert_ok(&response);
    let result: EpsgResponse = handle_json_response(response).await.unwrap();
    assert_eq!(result.srid, 27700);
}
