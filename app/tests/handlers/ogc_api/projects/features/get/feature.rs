use app::constants::TURBINE_LAYOUTS_COLLECTION_ID;
use domain::{FeatureId, ProjectCollectionId, ProjectFeature, ProjectId};

use crate::common::{
    AppBuilder, Auth, TestApp,
    helpers::{assert_ok, assert_status, handle_json_response},
};
use domain::TeamId;

pub fn check_ogc_feature_is_project_feature(ogc_feature: ogc::Feature) {
    let _project_feature = ProjectFeature::try_from(ogc_feature)
        .expect("failed to convert ogc feature to project feature");
}

#[actix_web::test]
async fn get_feature_works() {
    let app = TestApp::spawn_with_db().await;
    let auth = Auth::mock_session_token();
    let project_id = app.generate_project_id(Some(&auth)).await;
    let collection_id = app.generate_project_collection_id().await;

    let feature_id = app
        .generate_project_feature_id(collection_id, project_id, Some(&auth))
        .await;

    let response = app
        .ogc_service
        .get_project_feature(&app.api_client, project_id, collection_id, FeatureId(1))
        .await;

    assert_ok(&response);

    let ogc_feature: ogc::Feature = handle_json_response(response)
        .await
        .expect("failed to retrieve feature");

    check_ogc_feature_is_project_feature(ogc_feature.clone());

    let feature: domain::ProjectFeature = ogc_feature.try_into().unwrap();

    assert_eq!(feature.id, feature_id.feature_id.0);

    assert_eq!(feature.properties.collection_id, feature_id.collection_id.0);
}

#[actix_web::test]
async fn get_feature_works_for_turbine_layout() {
    let app = AppBuilder::new().build().await;
    let user = Auth::MockUserCredentials(app.generate_user(false, TeamId(0)).await);
    let project_id = app.generate_project_id(Some(&user)).await;
    let layout_id = app
        .generate_primary_layout_id(&project_id, Some(&user))
        .await;

    let response = app
        .ogc_service
        .get_project_feature(
            &app.api_client,
            project_id,
            ProjectCollectionId(TURBINE_LAYOUTS_COLLECTION_ID),
            FeatureId(layout_id.0),
        )
        .await;

    assert_ok(&response);
}

#[actix_web::test]
async fn get_turbine_layout_feature_returns_404_for_layout_belonging_to_different_project() {
    let app = AppBuilder::new().build().await;
    let user = Auth::MockUserCredentials(app.generate_user(false, TeamId(0)).await);
    let project_a = app.generate_project_id(Some(&user)).await;
    let project_b = app.generate_project_id(Some(&user)).await;
    let layout_id = app
        .generate_primary_layout_id(&project_a, Some(&user))
        .await;

    let response = app
        .ogc_service
        .get_project_feature(
            &app.api_client,
            project_b,
            ProjectCollectionId(TURBINE_LAYOUTS_COLLECTION_ID),
            FeatureId(layout_id.0),
        )
        .await;
    assert_status(&response, 404);
}

#[actix_web::test]
async fn get_turbine_layout_feature_returns_correct_ids() {
    let app = AppBuilder::new().build().await;
    let user = Auth::MockUserCredentials(app.generate_user(false, TeamId(0)).await);
    let project_id = app.generate_project_id(Some(&user)).await;
    let layout_id = app
        .generate_primary_layout_id(&project_id, Some(&user))
        .await;

    let response = app
        .ogc_service
        .get_project_feature(
            &app.api_client,
            project_id,
            ProjectCollectionId(TURBINE_LAYOUTS_COLLECTION_ID),
            FeatureId(layout_id.0),
        )
        .await;

    let feature: ogc::Feature = handle_json_response(response)
        .await
        .expect("failed to retrieve feature");

    assert_eq!(
        feature.properties.get("id"),
        Some(&serde_json::json!(layout_id.0))
    );
    assert_eq!(
        feature.properties.get("collection_id"),
        Some(&serde_json::json!(TURBINE_LAYOUTS_COLLECTION_ID))
    );
}

#[actix_web::test]
async fn get_project_feature_returns_404_for_project_not_found() {
    let app = TestApp::spawn_with_db().await;
    let auth = Auth::mock_session_token();
    let project_id = app.generate_project_id(Some(&auth)).await;
    let collection_id = app.generate_project_collection_id().await;
    let feature_id = app
        .generate_project_feature_id(collection_id, project_id, Some(&auth))
        .await;

    let response = app
        .ogc_service
        .get_project_feature(
            &app.api_client,
            ProjectId::default(),
            collection_id,
            feature_id.feature_id,
        )
        .await;
    assert_status(&response, 404);
}

#[actix_web::test]
async fn get_project_feature_returns_404_for_feature_belonging_to_different_project() {
    let app = TestApp::spawn_with_db().await;
    let auth = Auth::mock_session_token();
    let project_id = app.generate_project_id(Some(&auth)).await;
    let collection_id = app.generate_project_collection_id().await;
    let another_project = app.generate_project_id(Some(&auth)).await;
    let feature_id = app
        .generate_project_feature_id(collection_id, another_project, Some(&auth))
        .await;

    let response = app
        .ogc_service
        .get_project_feature(
            &app.api_client,
            project_id.into(),
            collection_id,
            feature_id.feature_id,
        )
        .await;
    assert_status(&response, 404);
}
