use app::handlers::ogc_api::features::patch::project_feature::PatchProjectFeaturePayload;
use domain::enums::Status;

use crate::common::{
    AppBuilder, Auth,
    helpers::{assert_status, handle_json_response},
};

#[actix_web::test]
async fn patch_project_feature_works() {
    let app = AppBuilder::new().build().await;
    let auth = Auth::mock_session_token();
    let collection_id = app.generate_project_collection_id(Some(&auth)).await;
    let (_, user_id, project_id) = app.generate_ids().await;
    let feature_id = app
        .generate_project_feature_id(collection_id, project_id, user_id, Some({}))
        .await;
    let mut feature: ogc::Feature = handle_json_response(
        app.ogc_service
            .get_project_feature(&app.api_client, project_id, collection_id, feature_id.id)
            .await,
    )
    .await
    .expect("failed to retrieve feature");
    let status: Status =
        serde_json::from_value(feature.properties.remove("status").expect("no status"))
            .expect("failed to deserialize");
    assert_eq!(status, Status::Active);
    let mut payload = PatchProjectFeaturePayload::default();
    payload.status = Some(Status::Archived);
    let response = app
        .ogc_service
        .patch_project_feature(
            &app.api_client,
            &payload,
            project_id,
            feature_id,
            Some(&auth),
        )
        .await;
    assert_status(&response, 204);
    let mut feature: ogc::Feature = handle_json_response(
        app.ogc_service
            .get_project_feature(&app.api_client, project_id, collection_id, feature_id.id)
            .await,
    )
    .await
    .expect("failed to retrieve feature");
    let status: Status =
        serde_json::from_value(feature.properties.remove("status").expect("no status"))
            .expect("failed to deserialize");
    assert_eq!(status, Status::Archived);
}
