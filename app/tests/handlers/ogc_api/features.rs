use serde::{Deserialize, Serialize};

use crate::common::{
    TestApp,
    helpers::{assert_ok, assert_status, check_ogc_feature, handle_json_response},
};

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Properties {
    some_text: String,
}

impl Default for Properties {
    fn default() -> Self {
        Self {
            some_text: uuid::Uuid::new_v4().to_string(),
        }
    }
}

#[actix_web::test]
async fn get_features_works() {
    let app = TestApp::spawn_with_db().await;
    let (_, user_id, project_id) = app.generate_ids().await;
    let (slug, collection_id) = app.generate_collection_slug_and_id(user_id).await;
    let _feature_id = app
        .generate_feature_id(
            collection_id,
            project_id,
            user_id,
            Some(Properties::default()),
        )
        .await;
    let response = app.ogc_service.get_features(&app.api_client, &slug).await;

    assert_ok(&response);

    let feature_collection: ogc::FeatureCollection = handle_json_response(response)
        .await
        .expect("failed to retrieve feature collection");

    assert_eq!(feature_collection.features.len(), 1);
    let ogc_feature = feature_collection.features.into_iter().next().unwrap();
    check_ogc_feature::<Properties>(ogc_feature);
}

#[actix_web::test]
async fn get_features_works_with_limit() {
    let app = TestApp::spawn_with_db().await;
    let (_, user_id, project_id) = app.generate_ids().await;
    let (slug, collection_id) = app.generate_collection_slug_and_id(user_id).await;
    for _ in 0..10 {
        app.generate_feature_id(
            collection_id,
            project_id,
            user_id,
            Some(Properties::default()),
        )
        .await;
    }
    let limit = 5;

    let response = app
        .ogc_service
        .get_features_with_params(&app.api_client, &slug, &[("limit", limit)])
        .await;
    assert_ok(&response);
    let feature_collection: ogc::FeatureCollection = handle_json_response(response)
        .await
        .expect("Failed to retrieve feature collection");
    assert_eq!(feature_collection.features.len(), limit);
    for ogc_feature in feature_collection.features {
        check_ogc_feature::<Properties>(ogc_feature);
    }
}

// We need to test the scenarion where two features have the same id but different collection  id
#[actix_web::test]
async fn get_feature_works() {
    let app = TestApp::spawn_with_db().await;
    let (_, user_id, project_id) = app.generate_ids().await;
    let (slug_1, collection_1_id) = app.generate_collection_slug_and_id(user_id).await;
    let (slug_2, collection_2_id) = app.generate_collection_slug_and_id(user_id).await;

    let feature_1_id = app
        .insert_feature_with_id(
            1,
            project_id,
            collection_1_id,
            user_id,
            Some(Properties::default()),
        )
        .await;
    let feature_2_id = app
        .insert_feature_with_id(
            1,
            project_id,
            collection_2_id,
            user_id,
            Some(Properties::default()),
        )
        .await;
    let response_1 = app
        .ogc_service
        .get_feature(&app.api_client, &slug_1, 1)
        .await;

    let response_2 = app
        .ogc_service
        .get_feature(&app.api_client, &slug_2, 1)
        .await;

    assert_ok(&response_1);
    assert_ok(&response_2);

    let ogc_feature_1: ogc::Feature = handle_json_response(response_1)
        .await
        .expect("failed to retrieve feature");

    let ogc_feature_2: ogc::Feature = handle_json_response(response_2)
        .await
        .expect("failed to retrieve feature");

    check_ogc_feature::<Properties>(ogc_feature_1.clone());
    check_ogc_feature::<Properties>(ogc_feature_2.clone());
    let feature_1: domain::ProjectFeature = ogc_feature_1.try_into().unwrap();
    let feature_2: domain::ProjectFeature = ogc_feature_2.try_into().unwrap();
    assert_eq!(feature_1.id, feature_1_id.id);
    assert_eq!(feature_2.id, feature_2_id.id);
    assert_eq!(
        feature_1.properties.collection_id,
        feature_1_id.collection_id.0
    );
    assert_eq!(
        feature_2.properties.collection_id,
        feature_2_id.collection_id.0
    );
}

#[actix_web::test]
async fn get_features_returns_empty_vec_for_no_features_in_collection() {
    let app = TestApp::spawn_with_db().await;
    let (_, user_id, _) = app.generate_ids().await;
    let (slug, _) = app.generate_collection_slug_and_id(user_id).await;
    let response = app.ogc_service.get_features(&app.api_client, &slug).await;
    assert_ok(&response);

    let feature_collection: ogc::FeatureCollection = handle_json_response(response)
        .await
        .expect("failed to retrieve feature collection");
    assert!(feature_collection.features.is_empty())
}

#[actix_web::test]
async fn get_feature_returns_404_for_non_existent_feature() {
    let app = TestApp::spawn_with_db().await;
    let (_, user_id, _) = app.generate_ids().await;
    let (slug, _) = app.generate_collection_slug_and_id(user_id).await;
    let response = app.ogc_service.get_feature(&app.api_client, &slug, 0).await;
    assert_status(&response, 404);
}
