use crate::common::{TestApp, helpers::check_error_response};

#[actix_web::test]
pub async fn unsupported_crs_in_request_returns_400() {
    let app = TestApp::spawn_with_db().await;
    let (_, user_id, project_id) = app.generate_ids().await;
    let (slug, collection_id) = app.generate_collection_slug_and_id(user_id).await;
    let srid = 9999;
    let feature = app
        .generate_feature_id(collection_id, project_id, user_id, Some({}))
        .await;
    let crs = format!("http://www.opengis.net/def/crs/EPSG/0/{srid}");

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
