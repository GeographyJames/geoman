use crate::common::{TestApp, helpers::assert_status};

#[actix_web::test]
pub async fn unsupported_crs_in_request_returns_400() {
    let app = TestApp::spawn_with_db().await;
    let (_, user_id, project_id) = app.generate_ids().await;
    let (slug, collection_id) = app.generate_collection_slug_and_id(user_id).await;
    let feature = app
        .generate_feature_id(collection_id, project_id, user_id, Some({}))
        .await;
    let crs = "http://www.opengis.net/def/crs/0/unsupported".to_string();
    let response = app
        .ogc_service
        .get_feature_with_params(&app.api_client, &slug, feature.id, &[("crs", crs)])
        .await;
    assert_status(&response, 400);
}
