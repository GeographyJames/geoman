use domain::Project;
use domain::enums::CollectionId;

use crate::common::{
    TestApp,
    helpers::{assert_ok, handle_json_response},
};

#[actix_web::test]
async fn get_projects_works() {
    let app = TestApp::spawn_with_db().await;
    let _ids = app.generate_ids().await;
    let response = app
        .ogc_service
        .get_features(&app.api_client, &CollectionId::Projects.to_string())
        .await;
    assert_ok(&response);
    let ogc_feature_collection: ogc::FeatureCollection = handle_json_response(response)
        .await
        .expect("failed to retrieve projects");
    let _projects = ogc_feature_collection
        .features
        .into_iter()
        .map(|f| Project::try_from(f).expect("Failed to convert to project"))
        .collect::<Vec<Project>>();
}

#[actix_web::test]
async fn get_project_works() {
    let app = TestApp::spawn_with_db().await;
    let (_, _, project_id) = app.generate_ids().await;
    let response = app
        .ogc_service
        .get_feature(
            &app.api_client,
            &CollectionId::Projects.to_string(),
            project_id.0,
        )
        .await;
    assert_ok(&response);
    let ogc_feature: ogc::Feature = handle_json_response(response)
        .await
        .expect("failed to retrieve project");
    let _project = Project::try_from(ogc_feature).expect("Failed to convert to project");
}

#[actix_web::test]
async fn get_projects_works_with_limit() {
    let app = TestApp::spawn_with_db().await;
    let team_id = app.generate_team_id().await;
    let user_id = app.generate_user_id(team_id).await;
    for _ in 0..10 {
        app.generate_project_id(user_id).await;
    }
    let limit = 5;

    let response = app
        .ogc_service
        .get_features_with_params(
            &app.api_client,
            &CollectionId::Projects.to_string(),
            &&[("limit", limit)],
        )
        .await;
    assert_ok(&response);
    let feature_collection: ogc::FeatureCollection = handle_json_response(response)
        .await
        .expect("Failed to retrieve projects");
    assert_eq!(feature_collection.features.len(), limit)
}
