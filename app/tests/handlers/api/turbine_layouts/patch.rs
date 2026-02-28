use app::{
    constants::TURBINE_LAYOUTS_COLLECTION_ID,
    handlers::api::features::patch::PatchProjectFeaturePayload,
};
use domain::{FeatureId, ProjectCollectionId, TeamId, enums::Status};

use crate::common::{
    AppBuilder, Auth,
    helpers::{assert_status, handle_json_response},
};

#[tokio::test]
async fn patch_turbine_layout_works() {
    let app = AppBuilder::new().build().await;
    let auth = Auth::MockUserCredentials(app.generate_user(false, TeamId(0)).await);
    let project_id = app.generate_project_id(Some(&auth)).await;
    let layout_id = app
        .generate_primary_layout_id(&project_id, Some(&auth))
        .await;

    let new_name = uuid::Uuid::new_v4().to_string();
    let update_dto = PatchProjectFeaturePayload {
        status: Some(Status::Archived),
        primary: Some(false),
        name: Some(new_name.clone()),
    };
    let response = app
        .features_service
        .patch_json(
            &app.api_client,
            format!(
                "{}/{}/{}",
                project_id, TURBINE_LAYOUTS_COLLECTION_ID, layout_id.0
            ),
            Some(&auth),
            &update_dto,
        )
        .await;
    assert_status(&response, 204);

    let mut feature: ogc::Feature = handle_json_response(
        app.ogc_service
            .get_project_feature(
                &app.api_client,
                project_id,
                ProjectCollectionId(TURBINE_LAYOUTS_COLLECTION_ID),
                FeatureId(layout_id.0),
            )
            .await,
    )
    .await
    .expect("failed to retrieve feature");

    let name = feature.properties.remove("name").expect("no name");
    assert_eq!(name, new_name);

    let status: Status =
        serde_json::from_value(feature.properties.remove("status").expect("no status"))
            .expect("failed to deserialize status");
    assert_eq!(status, Status::Archived);

    let is_primary = feature
        .properties
        .remove("is_primary")
        .expect("no is_primary");
    assert_eq!(is_primary, false);
}
