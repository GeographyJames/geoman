use app::{
    constants::TURBINE_LAYOUTS_COLLECTION_ID,
    handlers::api::features::duplicate::DuplicateLayoutBody,
};
use domain::{FeatureId, LayoutId, TeamId};

use crate::common::{
    AppBuilder, Auth,
    helpers::{assert_status, handle_json_response},
};

#[tokio::test]
async fn duplicate_turbine_layout_works() {
    let app = AppBuilder::new().build().await;
    let auth = Auth::MockUserCredentials(app.generate_user(false, TeamId(0)).await);
    let project_id = app.generate_project_id(Some(&auth)).await;
    let original_layout = app
        .generate_primary_layout_id(&project_id, Some(&auth))
        .await;
    let original_layout: ogc::Feature = handle_json_response(
        app.ogc_service
            .get_project_feature(
                &app.api_client,
                project_id,
                domain::ProjectCollectionId(TURBINE_LAYOUTS_COLLECTION_ID),
                FeatureId(original_layout.0),
            )
            .await,
    )
    .await
    .expect("failed to deserialize json");
    let updated_layout = DuplicateLayoutBody {
        name: Some(uuid::Uuid::new_v4().to_string()),
        hub_height_metre: None,
        rotor_diameter_metre: None,
        primary: None,
    };
    let response = app
        .duplicate_feature(
            Some(&auth),
            &updated_layout,
            project_id,
            domain::ProjectCollectionId(TURBINE_LAYOUTS_COLLECTION_ID),
            original_layout.id,
        )
        .await;
    assert_status(&response, 201);
    let _duplicate_id: LayoutId = handle_json_response(response)
        .await
        .expect("failed to retrieve duplicate layout id");
}
