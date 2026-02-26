use gdal::vector::Geometry;
use ogcapi_types::common::Crs;

use crate::common::{Auth, TestApp, helpers::handle_json_response};

#[actix_web::test]
pub async fn bbox_works() {
    let app = TestApp::spawn_with_db().await;
    let auth = Auth::mock_session_token();
    let project_id = app.generate_project_id(Some(&auth)).await;
    let collection_id = app.generate_project_collection_id().await;
    let srid = 27700;
    let feature_1 = format!("POINT({} {})", 1, 1);
    let feature_2 = format!("POINT({} {})", 3, 1);
    let _feature_1_id = app
        .post_project_feature(
            collection_id,
            project_id,
            Geometry::from_wkt(&feature_1).expect("failed to generate geom"),
            srid,
            Some(&auth),
            None,
        )
        .await;
    let _feature_2_id = app
        .post_project_feature(
            collection_id,
            project_id,
            Geometry::from_wkt(&feature_2).expect("failed to generate geom"),
            srid,
            Some(&auth),
            None,
        )
        .await;
    let bbox = ogcapi_types::common::Bbox::Bbox2D([0., 0., 2., 2.]).to_string();
    let response = app
        .ogc_service
        .get_project_features_with_params(
            &app.api_client,
            collection_id,
            project_id,
            &[
                ("bbox", bbox),
                ("bbox-crs", Crs::from_epsg(27700).to_string()),
            ],
        )
        .await;
    let features: ogc::FeatureCollection = handle_json_response(response)
        .await
        .expect("failed to extract features");
    assert_eq!(features.features.len(), 1);
}
