use crate::common::{
    TestApp,
    helpers::{generate_random_bng_point_ewkt, handle_json_response},
};
use ogc::FeatureCollection;
use ogcapi_types::common::Crs;

#[actix_web::test]
pub async fn crs_transform_works() {
    let app = TestApp::spawn_with_db().await;
    let (_, user_id, project_id) = app.generate_ids().await;
    let collection_id = app.generate_project_collection_id(None).await;
    let (easting, northing, ewkt) = generate_random_bng_point_ewkt();
    let feature_id = app
        .insert_project_feature(
            &uuid::Uuid::new_v4().to_string(),
            collection_id,
            project_id,
            user_id,
            &ewkt,
            Some({}),
        )
        .await;
    let srid = 27700;

    let crs = Crs::from_epsg(srid).to_string();

    let response = app
        .ogc_service
        .get_project_feature_with_params(
            &app.api_client,
            project_id,
            collection_id,
            feature_id.id,
            &[("crs", &crs)],
        )
        .await;
    let ogc_feature: ogc::Feature = handle_json_response(response)
        .await
        .expect("failed to retrieve feature");
    check_point_geom(&ogc_feature, easting, northing);

    let response = app
        .ogc_service
        .get_project_features_with_params(
            &app.api_client,
            collection_id.into(),
            project_id,
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

    let response = app
        .ogc_service
        .get_project_feature_with_params(
            &app.api_client,
            project_id.try_into().unwrap(),
            collection_id,
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
            collection_id,
            project_id.try_into().unwrap(),
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

fn check_point_geom(ogc_feature: &ogc::Feature, easting: f32, northing: f32) {
    match ogc_feature
        .geometry
        .as_ref()
        .expect("feature has no geometry")
        .value
    {
        geojson::Value::Point(ref items) => {
            assert_eq!(items.len(), 2);
            let x = (items[0] * 10.0).round() / 10.0;
            let y = (items[1] * 10.0).round() / 10.0;
            let expected_x = ((easting as f64) * 10.0).round() / 10.0;
            let expected_y = ((northing as f64) * 10.0).round() / 10.0;

            assert_eq!(x, expected_x, "X coordinate mismatch");
            assert_eq!(y, expected_y, "Y coordinate mismatch");
        }
        _ => panic!("feature returned is not a point!"),
    };
}
