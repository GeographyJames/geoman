use domain::{ProjectFeature, ProjectId};
use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::common::{
    TestApp,
    helpers::{assert_ok, assert_status, handle_json_response},
};

pub fn check_ogc_feature_is_project_feature<P: DeserializeOwned>(ogc_feature: ogc::Feature) {
    let project_feature = ProjectFeature::try_from(ogc_feature)
        .expect("failed to convert ogc featuer to project feature");
    let _props: P =
        serde_json::from_value(serde_json::Value::Object(project_feature.properties_map))
            .expect("failed to deserialise feature properties to properties struct");
}

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

// We need to test the scenarion where two features have the same id but different collection  id
#[actix_web::test]
async fn get_feature_works() {
    let app = TestApp::spawn_with_db().await;
    let (_, user_id, project_id) = app.generate_ids().await;
    let collection_1_id = app.generate_project_collection_id(user_id).await;
    let collection_2_id = app.generate_project_collection_id(user_id).await;

    let feature_1_id = app
        .insert_project_feature_with_id(
            1,
            project_id,
            collection_1_id,
            user_id,
            Some(Properties::default()),
        )
        .await;
    let feature_2_id = app
        .insert_project_feature_with_id(
            1,
            project_id,
            collection_2_id,
            user_id,
            Some(Properties::default()),
        )
        .await;
    let response_1 = app
        .ogc_service
        .get_project_feature(&app.api_client, project_id, collection_1_id, 1)
        .await;

    let response_2 = app
        .ogc_service
        .get_project_feature(&app.api_client, project_id, collection_2_id, 1)
        .await;

    assert_ok(&response_1);
    assert_ok(&response_2);

    let ogc_feature_1: ogc::Feature = handle_json_response(response_1)
        .await
        .expect("failed to retrieve feature");

    let ogc_feature_2: ogc::Feature = handle_json_response(response_2)
        .await
        .expect("failed to retrieve feature");

    check_ogc_feature_is_project_feature::<Properties>(ogc_feature_1.clone());
    check_ogc_feature_is_project_feature::<Properties>(ogc_feature_2.clone());
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
async fn get_project_feature_returns_404_for_project_not_found() {
    let app = TestApp::spawn_with_db().await;
    let (_, user_id, project_id) = app.generate_ids().await;
    let collection_id = app.generate_project_collection_id(user_id).await;
    let feature_id = app
        .generate_project_feature_id(collection_id, project_id, user_id, Some({}))
        .await;

    let response = app
        .ogc_service
        .get_project_feature(
            &app.api_client,
            ProjectId::default(),
            collection_id,
            feature_id.id,
        )
        .await;
    assert_status(&response, 404);
}

#[actix_web::test]
async fn get_project_feature_returns_404_for_feature_belonging_to_different_project() {
    let app = TestApp::spawn_with_db().await;
    let (_, user_id, project_id) = app.generate_ids().await;
    let collection_id = app.generate_project_collection_id(user_id).await;
    let another_project = app.generate_project_id(None).await;
    let feature_id = app
        .generate_project_feature_id(collection_id, another_project, user_id, Some({}))
        .await;

    let response = app
        .ogc_service
        .get_project_feature(
            &app.api_client,
            project_id.into(),
            collection_id,
            feature_id.id,
        )
        .await;
    assert_status(&response, 404);
}
