use geojson::{Feature, FeatureCollection};

use crate::app::{
    TestApp,
    helpers::{assert_ok, handle_json_response},
};

#[actix_web::test]
async fn get_features_works() {
    let app = TestApp::spawn_with_db().await;
    let team_id = app.generate_team_id().await;
    let user_id = app.generate_user_id(team_id).await;
    let (collection_slug, _) = app.generate_collection_slug_and_id(user_id).await;
    let response = app
        .ogc_service
        .get_features(&app.api_client, collection_slug.as_ref())
        .await;

    assert_ok(&response);

    let feature_collection: FeatureCollection = handle_json_response(response)
        .await
        .expect("failed to retrieve feature collection");

    assert_eq!(feature_collection.features.len(), 0);
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

    // Verify the feature has the correct ID
    assert_eq!(
        feature.id,
        Some(geojson::feature::Id::Number(feature_id.0.into()))
    );

    // Verify the feature has geometry
    assert!(feature.geometry.is_some());

    // Verify the feature has properties with a name
    assert!(feature.properties.is_some());
    let properties = feature.properties.unwrap();
    assert!(properties.contains_key("name"));
}
