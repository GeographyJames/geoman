use crate::common::{
    TestApp,
    helpers::{assert_ok, assert_status, handle_json_response},
};
use app::enums::ProjectIdentifier;

// Als ensures only features relating to the relevant project are returned
#[actix_web::test]
async fn get_features_works_for_project() {
    let app = TestApp::spawn_with_db().await;
    let (_, user_id, project_id) = app.generate_ids().await;
    let (slug, collection_id) = app.generate_collection_slug_and_id(user_id).await;
    let another_project = app.generate_project_id(user_id).await;
    let _feature_id = app
        .generate_feature_id(collection_id, project_id, user_id, Some({}))
        .await;
    let _other_project_feature = app
        .generate_feature_id(collection_id, another_project, user_id, Some({}))
        .await;
    let response = app
        .ogc_service
        .get_project_features(&app.api_client, &slug, &project_id.into(), None)
        .await;
    assert_ok(&response);
    let ogc_features: ogc::FeatureCollection = handle_json_response(response)
        .await
        .expect("failed to retrieve features");
    assert_eq!(ogc_features.features.len(), 1);
}

#[actix_web::test]
async fn get_feature_works_for_project() {
    let app = TestApp::spawn_with_db().await;
    let (_, user_id, project_id) = app.generate_ids().await;
    let (slug, collection_id) = app.generate_collection_slug_and_id(user_id).await;
    let feature_id = app
        .generate_feature_id(collection_id, project_id, user_id, Some({}))
        .await;

    let response = app
        .ogc_service
        .get_project_feature(&app.api_client, &project_id.into(), &slug, feature_id.id)
        .await;
    assert_ok(&response)
}

#[actix_web::test]
async fn get_feature_returns_404_for_project_not_found() {
    let app = TestApp::spawn_with_db().await;
    let (_, user_id, project_id) = app.generate_ids().await;
    let (slug, collection_id) = app.generate_collection_slug_and_id(user_id).await;
    let feature_id = app
        .generate_feature_id(collection_id, project_id, user_id, Some({}))
        .await;

    let response = app
        .ogc_service
        .get_project_feature(
            &app.api_client,
            &ProjectIdentifier::default(),
            &slug,
            feature_id.id,
        )
        .await;
    assert_status(&response, 404);
}

#[actix_web::test]
async fn get_feature_returns_404_for_feature_belonging_to_different_project() {
    let app = TestApp::spawn_with_db().await;
    let (_, user_id, project_id) = app.generate_ids().await;
    let (slug, collection_id) = app.generate_collection_slug_and_id(user_id).await;
    let another_project = app.generate_project_id(user_id).await;
    let feature_id = app
        .generate_feature_id(collection_id, another_project, user_id, Some({}))
        .await;

    let response = app
        .ogc_service
        .get_project_feature(&app.api_client, &project_id.into(), &slug, feature_id.id)
        .await;
    assert_status(&response, 404);
}

#[actix_web::test]
async fn get_features_returns_404_for_project_not_found() {
    let app = TestApp::spawn_with_db().await;
    let (_, user_id, project_id) = app.generate_ids().await;
    let (slug, collection_id) = app.generate_collection_slug_and_id(user_id).await;
    let _feature_id = app
        .generate_feature_id(collection_id, project_id, user_id, Some({}))
        .await;
    let response = app
        .ogc_service
        .get_project_features(&app.api_client, &slug, &ProjectIdentifier::default(), None)
        .await;
    assert_status(&response, 404);
}

#[actix_web::test]
async fn get_project_features_works_with_limit() {
    let app = TestApp::spawn_with_db().await;
    let (_, user_id, project_id) = app.generate_ids().await;
    let (collection_slug, collection_id) = app.generate_collection_slug_and_id(user_id).await;
    for _ in 0..10 {
        let _f = app
            .generate_feature_id(collection_id, project_id, user_id, Some({}))
            .await;
    }
    let limit = 5;
    let params = ogc::features::Query {
        limit: Some(limit),
        ..Default::default()
    };
    let response = app
        .ogc_service
        .get_project_features(
            &app.api_client,
            &collection_slug,
            &project_id.into(),
            Some(&params),
        )
        .await;
    let features: ogc::FeatureCollection = handle_json_response(response)
        .await
        .expect("failed to retrieve features");
    assert_eq!(features.features.len(), limit)
}
