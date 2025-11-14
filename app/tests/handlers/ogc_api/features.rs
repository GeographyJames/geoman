use domain::FeatureId;
use ogc::types::{Feature, FeatureCollection, features::Query};
use serde::{Deserialize, Serialize};

use crate::common::{
    TestApp,
    helpers::{assert_ok, assert_status, check_feature, handle_json_response},
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
    let response = app
        .ogc_service
        .get_features(&app.api_client, &slug, None)
        .await;

    assert_ok(&response);

    let feature_collection: FeatureCollection = handle_json_response(response)
        .await
        .expect("failed to retrieve feature collection");

    assert_eq!(feature_collection.features.len(), 1);
    let feature = feature_collection.features.iter().next().unwrap();
    check_feature::<Properties>(feature);
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
    let params = Query {
        limit: Some(limit),
        ..Default::default()
    };
    let response = app
        .ogc_service
        .get_features(&app.api_client, &slug, Some(&params))
        .await;
    assert_ok(&response);
    let feature_collection: FeatureCollection = handle_json_response(response)
        .await
        .expect("Failed to retrieve feature collection");
    assert_eq!(feature_collection.features.len(), limit);
    for ft in feature_collection.features {
        check_feature::<Properties>(&ft);
    }
}

#[actix_web::test]
async fn get_feature_works() {
    let app = TestApp::spawn_with_db().await;
    let (_, user_id, project_id) = app.generate_ids().await;
    let (slug, collection_id) = app.generate_collection_slug_and_id(user_id).await;
    let feature_id = app
        .generate_feature_id(
            collection_id,
            project_id,
            user_id,
            Some(Properties::default()),
        )
        .await;
    let response = app
        .ogc_service
        .get_feature(&app.api_client, &slug, feature_id)
        .await;

    assert_ok(&response);

    let feature: Feature = handle_json_response(response)
        .await
        .expect("failed to retrieve feature");
    check_feature::<Properties>(&feature);
}

#[actix_web::test]
async fn get_features_returns_empty_vec_for_no_features_in_collection() {
    let app = TestApp::spawn_with_db().await;
    let (_, user_id, _) = app.generate_ids().await;
    let (slug, _) = app.generate_collection_slug_and_id(user_id).await;
    let response = app
        .ogc_service
        .get_features(&app.api_client, &slug, None)
        .await;
    assert_ok(&response);

    let feature_collection: FeatureCollection = handle_json_response(response)
        .await
        .expect("failed to retrieve feature collection");
    assert!(feature_collection.features.is_empty())
}

#[actix_web::test]
async fn get_feature_returns_404_for_non_existent_feature() {
    let app = TestApp::spawn_with_db().await;
    let (_, user_id, _) = app.generate_ids().await;
    let (slug, _) = app.generate_collection_slug_and_id(user_id).await;
    let response = app
        .ogc_service
        .get_feature(&app.api_client, &slug, FeatureId::default())
        .await;
    assert_status(&response, 404);
}
