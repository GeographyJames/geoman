use domain::ProjectId;

use crate::common::{
    Auth, TestApp,
    helpers::{assert_ok, assert_status, handle_json_response},
};

// Als ensures only features relating to the relevant project are returned
#[actix_web::test]
async fn get_project_features_works() {
    let app = TestApp::spawn_with_db().await;
    let auth = Auth::mock_session_token();
    let (_, user_id, project_id) = app.generate_ids().await;
    let collection_id = app.generate_project_collection_id(Some(&auth)).await;
    let another_project = app.generate_project_id(Some(&auth)).await;
    let _feature_id = app
        .generate_project_feature_id(collection_id, project_id, user_id, Some({}))
        .await;
    let _other_project_feature = app
        .generate_project_feature_id(collection_id, another_project, user_id, Some({}))
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
    let (_, user_id, project_id) = app.generate_ids().await;
    let collection_id = app.generate_project_collection_id(Some(&auth)).await;
    let _feature_id = app
        .generate_project_feature_id(collection_id, project_id, user_id, Some({}))
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
    let (_, user_id, project_id) = app.generate_ids().await;
    let collection_id = app.generate_project_collection_id(Some(&auth)).await;
    for _ in 0..10 {
        let _f = app
            .generate_project_feature_id(collection_id, project_id, user_id, Some({}))
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
