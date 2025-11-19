use crate::common::{
    TestApp,
    helpers::{check_error_response, generate_random_bng_point_ewkt, handle_json_response},
};
use ogc::FeatureCollection;
use ogcapi_types::common::Crs;

#[actix_web::test]
pub async fn unsupported_crs_in_request_returns_400() {
    let app = TestApp::spawn_with_db().await;
    let (_, user_id, project_id) = app.generate_ids().await;
    let (slug, collection_id) = app.generate_collection_slug_and_id(user_id).await;
    let srid = 9999;
    let feature = app
        .generate_feature_id(collection_id, project_id, user_id, Some({}))
        .await;
    let crs = Crs::from_epsg(9999).to_string();

    // Get feature
    let response = app
        .ogc_service
        .get_feature_with_params(&app.api_client, &slug, feature.id, &[("crs", &crs)])
        .await;
    check_error_response(response, 400, &format!("Unsupported CRS: {}", srid)).await;

    // Get project feature
    let response = app
        .ogc_service
        .get_features_with_params(&app.api_client, &slug, &[("crs", &crs)])
        .await;
    check_error_response(response, 400, &format!("Unsupported CRS: {}", srid)).await;

    // Get features
    let response = app
        .ogc_service
        .get_project_feature_with_params(
            &app.api_client,
            &project_id.into(),
            &slug,
            feature.id,
            &[("crs", &crs)],
        )
        .await;
    check_error_response(response, 400, &format!("Unsupported CRS: {}", srid)).await;

    // Get project features
    let response = app
        .ogc_service
        .get_project_features_with_params(
            &app.api_client,
            &slug,
            &project_id.into(),
            &[("crs", crs)],
        )
        .await;
    check_error_response(response, 400, &format!("Unsupported CRS: {}", srid)).await;
}

#[actix_web::test]
pub async fn crs_transform_works() {
    let app = TestApp::spawn_with_db().await;
    let (_, user_id, project_id) = app.generate_ids().await;
    let (slug, collection_id) = app.generate_collection_slug_and_id(user_id).await;
    let (easting, northing, ewkt) = generate_random_bng_point_ewkt();
    let feature_id = app
        .insert_feature(
            &uuid::Uuid::new_v4().to_string(),
            collection_id,
            project_id,
            user_id,
            &ewkt,
            Some({}),
        )
        .await;
    let srid = 27700;
    app.insert_crs(srid).await;
    let crs = Crs::from_epsg(srid).to_string();

    let response = app
        .ogc_service
        .get_feature_with_params(&app.api_client, &slug, feature_id.id, &[("crs", &crs)])
        .await;
    let ogc_feature: ogc::Feature = handle_json_response(response)
        .await
        .expect("failed to retrieve feature");
    check_point_geom(&ogc_feature, easting, northing);

    let response = app
        .ogc_service
        .get_features_with_params(&app.api_client, &slug, &[("crs", &crs)])
        .await;
    let ogc_features: FeatureCollection = handle_json_response(response)
        .await
        .expect("failed to retrieve features");

    check_point_geom(
        ogc_features
            .features
            .iter()
            .next()
            .expect("no features returned"),
        easting,
        northing,
    );

    let response = app
        .ogc_service
        .get_project_feature_with_params(
            &app.api_client,
            &project_id.try_into().unwrap(),
            &slug,
            feature_id.id,
            &[("crs", &crs)],
        )
        .await;
    let ogc_feature: ogc::Feature = handle_json_response(response)
        .await
        .expect("failed to retrieve features");

    check_point_geom(&ogc_feature, easting, northing);

    let response = app
        .ogc_service
        .get_project_features_with_params(
            &app.api_client,
            &slug,
            &project_id.try_into().unwrap(),
            &[("crs", &crs)],
        )
        .await;
    let ogc_features: FeatureCollection = handle_json_response(response)
        .await
        .expect("failed to retrieve features");

    check_point_geom(
        ogc_features
            .features
            .iter()
            .next()
            .expect("no features returned"),
        easting,
        northing,
    );
}

fn check_point_geom(ogc_feature: &ogc::Feature, easting: u32, northing: u32) {
    match ogc_feature
        .geometry
        .as_ref()
        .expect("feature has no geometry")
        .value
    {
        geojson::Value::Point(ref items) => {
            assert_eq!(items, &[easting as f64, northing as f64]);
        }
        _ => panic!("feature returned is not a point!"),
    };
}
