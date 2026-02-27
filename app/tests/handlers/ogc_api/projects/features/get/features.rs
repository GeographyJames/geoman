use app::constants::TURBINE_LAYOUTS_COLLECTION_ID;
use domain::{ProjectCollectionId, ProjectId};

use crate::common::{
    Auth, TestApp,
    helpers::{assert_ok, assert_status, handle_json_response},
};

// Als ensures only features relating to the relevant project are returned
#[actix_web::test]
async fn get_project_features_works() {
    let app = TestApp::spawn_with_db().await;
    let auth = Auth::mock_session_token();
    let project_id = app.generate_project_id(Some(&auth)).await;
    let collection_id = app.generate_project_collection_id().await;
    let another_project = app.generate_project_id(Some(&auth)).await;
    let _feature_id = app
        .generate_project_feature_id(collection_id, project_id, Some(&auth))
        .await;
    let _other_project_feature = app
        .generate_project_feature_id(collection_id, another_project, Some(&auth))
        .await;
    let response = app
        .ogc_service
        .get_project_features(&app.api_client, collection_id, project_id)
        .await;
    assert_ok(&response);
    let ogc_features: ogc::FeatureCollection = handle_json_response(response)
        .await
        .expect("failed to retrieve features");
    assert_eq!(ogc_features.features.len(), 1);
}

#[actix_web::test]
async fn get_projct_features_returns_404_for_project_not_found() {
    let app = TestApp::spawn_with_db().await;
    let auth = Auth::mock_session_token();
    let project_id = app.generate_project_id(Some(&auth)).await;
    let collection_id = app.generate_project_collection_id().await;
    let _feature_id = app
        .generate_project_feature_id(collection_id, project_id, Some(&auth))
        .await;
    let response = app
        .ogc_service
        .get_project_features(&app.api_client, collection_id, ProjectId::default())
        .await;
    assert_status(&response, 404);
}

#[actix_web::test]
async fn get_project_features_works_with_limit() {
    let app = TestApp::spawn_with_db().await;
    let auth = Auth::mock_session_token();
    let project_id = app.generate_project_id(Some(&auth)).await;
    let collection_id = app.generate_project_collection_id().await;
    for _ in 0..10 {
        let _f = app
            .generate_project_feature_id(collection_id, project_id, Some(&auth))
            .await;
    }
    let limit = 5;

    let response = app
        .ogc_service
        .get_project_features_with_params(
            &app.api_client,
            collection_id,
            project_id.into(),
            &&[("limit", limit)],
        )
        .await;
    let features: ogc::FeatureCollection = handle_json_response(response)
        .await
        .expect("failed to retrieve features");
    assert_eq!(features.features.len(), limit)
}

#[actix_web::test]
async fn get_turbine_layout_features_returns_404_when_project_has_no_layouts() {
    let app = TestApp::spawn_with_db().await;
    let auth = Auth::mock_session_token();
    let project_id = app.generate_project_id(Some(&auth)).await;
    let response = app
        .ogc_service
        .get_project_features(
            &app.api_client,
            ProjectCollectionId(TURBINE_LAYOUTS_COLLECTION_ID),
            project_id,
        )
        .await;
    assert_status(&response, 404);
}

#[actix_web::test]
async fn get_turbine_layout_features_only_returns_layouts_for_the_project() {
    let app = TestApp::spawn_with_db().await;
    let auth = Auth::mock_session_token();
    let project_a = app.generate_project_id(Some(&auth)).await;
    let project_b = app.generate_project_id(Some(&auth)).await;
    let _ = app.generate_primary_layout(&project_a, Some(&auth)).await;
    let _ = app.generate_primary_layout(&project_b, Some(&auth)).await;
    let response = app
        .ogc_service
        .get_project_features(
            &app.api_client,
            ProjectCollectionId(TURBINE_LAYOUTS_COLLECTION_ID),
            project_b,
        )
        .await;
    let ogc_features: ogc::FeatureCollection = handle_json_response(response)
        .await
        .expect("failed to retrieve features");
    assert_eq!(ogc_features.features.len(), 1);
}

#[actix_web::test]
async fn get_turbine_layout_features_returns_one_feature_per_layout() {
    let app = TestApp::spawn_with_db().await;
    let auth = Auth::mock_session_token();
    let project_id = app.generate_project_id(Some(&auth)).await;
    let _ = app.generate_primary_layout(&project_id, Some(&auth)).await;
    let _ = app.generate_primary_layout(&project_id, Some(&auth)).await;
    let response = app
        .ogc_service
        .get_project_features(
            &app.api_client,
            ProjectCollectionId(TURBINE_LAYOUTS_COLLECTION_ID),
            project_id,
        )
        .await;
    let ogc_features: ogc::FeatureCollection = handle_json_response(response)
        .await
        .expect("failed to retrieve features");
    assert_eq!(ogc_features.features.len(), 2);
}

#[actix_web::test]
async fn get_turbine_layout_features_works_with_limit() {
    let app = TestApp::spawn_with_db().await;
    let auth = Auth::mock_session_token();
    let project_id = app.generate_project_id(Some(&auth)).await;
    for _ in 0..3 {
        let _ = app.generate_primary_layout(&project_id, Some(&auth)).await;
    }
    let limit = 2;
    let response = app
        .ogc_service
        .get_project_features_with_params(
            &app.api_client,
            ProjectCollectionId(TURBINE_LAYOUTS_COLLECTION_ID),
            project_id.into(),
            &&[("limit", limit)],
        )
        .await;
    let ogc_features: ogc::FeatureCollection = handle_json_response(response)
        .await
        .expect("failed to retrieve features");
    assert_eq!(ogc_features.features.len(), limit);
}

#[actix_web::test]
async fn get_project_features_works_with_turbine_layouts() {
    let app = TestApp::spawn_with_db().await;
    let auth = Auth::mock_session_token();
    let project_id = app.generate_project_id(Some(&auth)).await;
    let _layout_id = app.generate_primary_layout(&project_id, Some(&auth)).await;
    let response = app
        .ogc_service
        .get_project_features(
            &app.api_client,
            ProjectCollectionId(TURBINE_LAYOUTS_COLLECTION_ID),
            project_id,
        )
        .await;
    let ogc_features: ogc::FeatureCollection = handle_json_response(response)
        .await
        .expect("failed to retrieve features");
    assert_eq!(ogc_features.features.len(), 1);
}
