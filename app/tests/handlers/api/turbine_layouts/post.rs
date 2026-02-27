use app::constants::TURBINE_LAYOUTS_COLLECTION_ID;
use domain::{FeatureId, ProjectCollectionId, TeamId};

use crate::common::{
    AppBuilder, Auth,
    helpers::{assert_ok, handle_json_response},
};

#[tokio::test]
async fn post_turbine_layout_works() {
    let app = AppBuilder::new().build().await;
    let auth = Auth::MockUserCredentials(app.generate_user(false, TeamId(0)).await);
    let project_id = app.generate_project_id(Some(&auth)).await;
    let response = app.generate_primary_layout(&project_id, Some(&auth)).await;
    assert_ok(&response);
    let feature_id: FeatureId = handle_json_response(response)
        .await
        .expect("failed to retrieve feature id");
    let response = app
        .ogc_service
        .get_project_feature(
            &app.api_client,
            project_id,
            ProjectCollectionId(TURBINE_LAYOUTS_COLLECTION_ID),
            feature_id,
        )
        .await;
    assert_ok(&response);
    let ogc_feature: ogc::Feature = handle_json_response(response)
        .await
        .expect("failed to retrieve feature");

    let feature: domain::ProjectFeature = ogc_feature.try_into().unwrap();
}
