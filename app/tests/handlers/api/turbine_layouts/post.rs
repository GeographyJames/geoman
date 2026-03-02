use app::constants::TURBINE_LAYOUTS_COLLECTION_ID;
use domain::{FeatureId, ProjectCollectionId, ProjectId, TeamId};
use serde_json::json;

use crate::common::{
    AppBuilder, Auth, TestApp,
    helpers::{TurbineInput, assert_ok, assert_status, handle_json_response},
    services::ClerkAuthService,
};

#[derive(serde::Deserialize)]
struct LayoutProps {
    hub_height_mm: serde_json::Value,
    rotor_diameter_mm: serde_json::Value,
}

async fn setup() -> (TestApp<ClerkAuthService>, Auth, ProjectId) {
    let app = AppBuilder::new().build().await;
    let auth = Auth::MockUserCredentials(app.generate_user(false, TeamId(0)).await);
    let project_id = app.generate_project_id(Some(&auth)).await;
    (app, auth, project_id)
}

async fn fetch_layout_props(
    app: &TestApp<ClerkAuthService>,
    auth: &Auth,
    project_id: ProjectId,
    turbines: &[TurbineInput],
    map_hub_height: bool,
    map_rotor_diameter: bool,
) -> LayoutProps {
    let response = app
        .post_turbine_layout(
            &project_id,
            turbines,
            map_hub_height,
            map_rotor_diameter,
            true,
            Some(auth),
        )
        .await;
    let feature_id: FeatureId = handle_json_response(response)
        .await
        .expect("failed to post layout");
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
    let feature: ogc::Feature = handle_json_response(response)
        .await
        .expect("failed to get layout");
    serde_json::from_value(serde_json::Value::Object(feature.properties))
        .expect("failed to deserialise layout properties")
}

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

    let _feature: domain::ProjectFeature = ogc_feature.try_into().unwrap();
}

// --- hub_height_mm ---

#[tokio::test]
async fn hub_height_is_single_value_when_all_turbines_match() {
    let (app, auth, project_id) = setup().await;
    let turbines = vec![
        TurbineInput { hub_m: Some(120.0), rd_m: None },
        TurbineInput { hub_m: Some(120.0), rd_m: None },
        TurbineInput { hub_m: Some(120.0), rd_m: None },
    ];
    let props = fetch_layout_props(&app, &auth, project_id, &turbines, true, false).await;
    assert_eq!(props.hub_height_mm, json!(120000));
}

#[tokio::test]
async fn hub_height_is_various_when_turbines_differ() {
    let (app, auth, project_id) = setup().await;
    let turbines = vec![
        TurbineInput { hub_m: Some(100.0), rd_m: None },
        TurbineInput { hub_m: Some(120.0), rd_m: None },
        TurbineInput { hub_m: Some(140.0), rd_m: None },
    ];
    let props = fetch_layout_props(&app, &auth, project_id, &turbines, true, false).await;
    assert_eq!(props.hub_height_mm, json!("various"));
}

#[tokio::test]
async fn hub_height_is_none_when_field_not_mapped() {
    let (app, auth, project_id) = setup().await;
    let turbines = vec![
        TurbineInput { hub_m: Some(120.0), rd_m: None },
        TurbineInput { hub_m: Some(120.0), rd_m: None },
        TurbineInput { hub_m: Some(120.0), rd_m: None },
    ];
    let props = fetch_layout_props(&app, &auth, project_id, &turbines, false, false).await;
    assert_eq!(props.hub_height_mm, json!(null));
}

#[tokio::test]
async fn hub_height_is_various_when_some_turbines_have_no_value() {
    let (app, auth, project_id) = setup().await;
    let turbines = vec![
        TurbineInput { hub_m: Some(120.0), rd_m: None },
        TurbineInput { hub_m: None, rd_m: None },
        TurbineInput { hub_m: None, rd_m: None },
    ];
    let props = fetch_layout_props(&app, &auth, project_id, &turbines, true, false).await;
    assert_eq!(props.hub_height_mm, json!("various"));
}

// --- rotor_diameter_mm ---

#[tokio::test]
async fn rotor_diameter_is_single_value_when_all_turbines_match() {
    let (app, auth, project_id) = setup().await;
    let turbines = vec![
        TurbineInput { hub_m: None, rd_m: Some(160.0) },
        TurbineInput { hub_m: None, rd_m: Some(160.0) },
        TurbineInput { hub_m: None, rd_m: Some(160.0) },
    ];
    let props = fetch_layout_props(&app, &auth, project_id, &turbines, false, true).await;
    assert_eq!(props.rotor_diameter_mm, json!(160000));
}

#[tokio::test]
async fn rotor_diameter_is_various_when_turbines_differ() {
    let (app, auth, project_id) = setup().await;
    let turbines = vec![
        TurbineInput { hub_m: None, rd_m: Some(150.0) },
        TurbineInput { hub_m: None, rd_m: Some(160.0) },
        TurbineInput { hub_m: None, rd_m: Some(170.0) },
    ];
    let props = fetch_layout_props(&app, &auth, project_id, &turbines, false, true).await;
    assert_eq!(props.rotor_diameter_mm, json!("various"));
}

#[tokio::test]
async fn rotor_diameter_is_various_when_some_turbines_have_no_value() {
    let (app, auth, project_id) = setup().await;
    let turbines = vec![
        TurbineInput { hub_m: None, rd_m: Some(160.0) },
        TurbineInput { hub_m: None, rd_m: None },
        TurbineInput { hub_m: None, rd_m: None },
    ];
    let props = fetch_layout_props(&app, &auth, project_id, &turbines, false, true).await;
    assert_eq!(props.rotor_diameter_mm, json!("various"));
}

#[tokio::test]
async fn rotor_diameter_is_none_when_field_not_mapped() {
    let (app, auth, project_id) = setup().await;
    let turbines = vec![
        TurbineInput { hub_m: None, rd_m: Some(160.0) },
        TurbineInput { hub_m: None, rd_m: Some(160.0) },
        TurbineInput { hub_m: None, rd_m: Some(160.0) },
    ];
    let props = fetch_layout_props(&app, &auth, project_id, &turbines, false, false).await;
    assert_eq!(props.rotor_diameter_mm, json!(null));
}

#[tokio::test]
async fn post_turbine_layout_with_no_turbines_returns_422() {
    let (app, auth, project_id) = setup().await;
    let response = app
        .post_turbine_layout(&project_id, &[], true, true, true, Some(&auth))
        .await;
    assert_status(&response, 422);
}
