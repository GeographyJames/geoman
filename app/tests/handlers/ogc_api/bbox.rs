use crate::common::{
    TestApp,
    helpers::{generate_point_bng, handle_json_response},
};

#[actix_web::test]
pub async fn bbox_works() {
    let app = TestApp::spawn_with_db().await;
    let (_, user_id, project_id) = app.generate_ids().await;
    let (slug, collection_id) = app.generate_collection_slug_and_id(user_id).await;
    let feature_1 = generate_point_bng(1., 1.);
    let feature_2 = generate_point_bng(3., 1.);
    let _feature_1_id = app
        .insert_feature(
            &uuid::Uuid::new_v4().to_string(),
            collection_id,
            project_id,
            user_id,
            &feature_1,
            Some({}),
        )
        .await;
    let _feature_2_id = app
        .insert_feature(
            &uuid::Uuid::new_v4().to_string(),
            collection_id,
            project_id,
            user_id,
            &feature_2,
            Some({}),
        )
        .await;
    let bbox = ogcapi_types::common::Bbox::Bbox2D([0., 0., 1., 2.]).to_string();
    let response = app
        .ogc_service
        .get_project_features_with_params(
            &app.api_client,
            &slug,
            &project_id.into(),
            &[("bbox", bbox)],
        )
        .await;
    let features: ogc::FeatureCollection = handle_json_response(response)
        .await
        .expect("failed to extract features");
    assert_eq!(features.features.len(), 1);
}
