use crate::common::{
    helpers::{assert_ok, handle_json_response},
    AppBuilder, Auth,
};
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
    let srs =
        gdal::spatial_ref::SpatialRef::from_epsg(4326).expect("failed to create spatial ref");
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
