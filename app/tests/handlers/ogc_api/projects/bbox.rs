use ogcapi_types::common::Crs;

use crate::common::{
    TestApp,
    helpers::{generate_point, handle_json_response},
};

#[actix_web::test]
pub async fn bbox_works() {
    let app = TestApp::spawn_with_db().await;

    let (_, user_id, project_id) = app.generate_ids().await;
    let collection_id = app.generate_project_collection_id(None).await;
    let feature_1 = generate_point(1., 1., 27700);
    let feature_2 = generate_point(3., 1., 27700);
    let _feature_1_id = app
        .insert_project_feature(
            &uuid::Uuid::new_v4().to_string(),
            collection_id,
            project_id,
            user_id,
            &feature_1,
            Some({}),
        )
        .await;
    let _feature_2_id = app
        .insert_project_feature(
            &uuid::Uuid::new_v4().to_string(),
            collection_id,
            project_id,
            user_id,
            &feature_2,
            Some({}),
        )
        .await;
    let bbox = ogcapi_types::common::Bbox::Bbox2D([0., 0., 2., 2.]).to_string();
    let response = app
        .ogc_service
        .get_project_features_with_params(
            &app.api_client,
            collection_id,
            project_id,
            &[
                ("bbox", bbox),
                ("bbox-crs", Crs::from_epsg(27700).to_string()),
            ],
        )
        .await;
    let features: ogc::FeatureCollection = handle_json_response(response)
        .await
        .expect("failed to extract features");
    assert_eq!(features.features.len(), 1);
}
