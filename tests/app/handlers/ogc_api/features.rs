use geojson::{Feature, FeatureCollection, feature};
use geoman::domain::FeatureId;

use crate::app::{
    TestApp,
    helpers::{assert_ok, handle_json_response},
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

fn check_feature(feature: &geojson::Feature, feature_id: FeatureId) {
    // Verify the feature has geometry
    assert!(feature.geometry.is_some(), "feature has no geometry");

    // Verify the feature has id that matches the expected feature_id
    let id = feature.id.as_ref().expect("feature has no id");

    match id {
        feature::Id::Number(number) => {
            let id_value = number.as_i64().expect("feature id is not a valid i64");
            assert_eq!(id_value, feature_id.0 as i64, "feature id does not match");
        }
        feature::Id::String(_) => panic!("feature id is a string, expected number"),
    }

    // Verify the feature has properties with a name
    let properties = feature
        .properties
        .as_ref()
        .expect("feature has no properties");
    assert!(
        properties.contains_key("name"),
        "properties has no name field"
    );
}
