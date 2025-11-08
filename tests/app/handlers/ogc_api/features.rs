use geojson::{Feature, FeatureCollection};

use crate::app::{
    TestApp,
    helpers::{assert_ok, assert_status, check_feature, handle_json_response},
};

#[actix_web::test]
async fn get_features_works() {
    let app = TestApp::spawn_with_db().await;
    let team_id = app.generate_team_id().await;
    let user_id = app.generate_user_id(team_id).await;
    let project_id = app.generate_project_id(user_id).await;
    let (collection_slug, collection_id) = app.generate_collection_slug_and_id(user_id).await;
    let feature_id = app
        .generate_feature_id(collection_id, project_id, user_id)
        .await;
    let response = app
        .ogc_service
        .get_features(&app.api_client, collection_slug.as_ref())
        .await;

    assert_ok(&response);

    let feature_collection: FeatureCollection = handle_json_response(response)
        .await
        .expect("failed to retrieve feature collection");

    assert_eq!(feature_collection.features.len(), 1);
    let feature = feature_collection.features.iter().next().unwrap();
    check_feature(feature, feature_id);

    assert_eq!(feature_collection.bbox, None);
}

#[actix_web::test]
async fn get_feature_works() {
    let app = TestApp::spawn_with_db().await;
    let team_id = app.generate_team_id().await;
    let user_id = app.generate_user_id(team_id).await;
    let project_id = app.generate_project_id(user_id).await;
    let (collection_slug, collection_id) = app.generate_collection_slug_and_id(user_id).await;
    let feature_id = app
        .generate_feature_id(collection_id, project_id, user_id)
        .await;
    let response = app
        .ogc_service
        .get_feature(&app.api_client, collection_slug.as_ref(), feature_id)
        .await;

    assert_ok(&response);

    let feature: Feature = handle_json_response(response)
        .await
        .expect("failed to retrieve feature");
    check_feature(&feature, feature_id);
}

#[actix_web::test]
async fn get_features_returns_404_for_non_existent_collection() {
    let app = TestApp::spawn_with_db().await;
    let response = app
        .ogc_service
        .get_features(&app.api_client, &uuid::Uuid::new_v4().to_string())
        .await;
    assert_status(&response, 404);
}
