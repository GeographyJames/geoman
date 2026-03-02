use app::constants::TURBINE_LAYOUTS_COLLECTION_ID;
use domain::{FeatureId, ProjectCollectionId, ProjectId};
use serde_json::json;

use crate::common::{
    Auth, TestApp,
    helpers::{TurbineInput, assert_ok, assert_status, handle_json_response},
    services::ClerkAuthService,
};

#[derive(serde::Deserialize)]
struct LayoutProps {
    hub_height_mm: serde_json::Value,
    rotor_diameter_mm: serde_json::Value,
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
async fn post_turbine_layout() {
    let (app, auth, project_id) = TestApp::with_project().await;

    // post works end-to-end
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

    // no turbines returns 422
    let response = app
        .post_turbine_layout(&project_id, &[], true, true, true, Some(&auth))
        .await;
    assert_status(&response, 422);

    // --- hub_height_mm ---

    let props = fetch_layout_props(
        &app,
        &auth,
        project_id,
        &[
            TurbineInput {
                hub_m: Some(120.0),
                rd_m: None,
            },
            TurbineInput {
                hub_m: Some(120.0),
                rd_m: None,
            },
            TurbineInput {
                hub_m: Some(120.0),
                rd_m: None,
            },
        ],
        true,
        false,
    )
    .await;
    assert_eq!(
        props.hub_height_mm,
        json!(120000),
        "single value when all turbines match"
    );

    let props = fetch_layout_props(
        &app,
        &auth,
        project_id,
        &[
            TurbineInput {
                hub_m: Some(100.0),
                rd_m: None,
            },
            TurbineInput {
                hub_m: Some(120.0),
                rd_m: None,
            },
            TurbineInput {
                hub_m: Some(140.0),
                rd_m: None,
            },
        ],
        true,
        false,
    )
    .await;
    assert_eq!(
        props.hub_height_mm,
        json!("various"),
        "various when turbines differ"
    );

    let props = fetch_layout_props(
        &app,
        &auth,
        project_id,
        &[
            TurbineInput {
                hub_m: Some(120.0),
                rd_m: None,
            },
            TurbineInput {
                hub_m: Some(120.0),
                rd_m: None,
            },
            TurbineInput {
                hub_m: Some(120.0),
                rd_m: None,
            },
        ],
        false,
        false,
    )
    .await;
    assert_eq!(
        props.hub_height_mm,
        json!(null),
        "null when field not mapped"
    );

    let props = fetch_layout_props(
        &app,
        &auth,
        project_id,
        &[
            TurbineInput {
                hub_m: Some(120.0),
                rd_m: None,
            },
            TurbineInput {
                hub_m: None,
                rd_m: None,
            },
            TurbineInput {
                hub_m: None,
                rd_m: None,
            },
        ],
        true,
        false,
    )
    .await;
    assert_eq!(
        props.hub_height_mm,
        json!("various"),
        "various when some turbines have no value"
    );

    // --- rotor_diameter_mm ---

    let props = fetch_layout_props(
        &app,
        &auth,
        project_id,
        &[
            TurbineInput {
                hub_m: None,
                rd_m: Some(160.0),
            },
            TurbineInput {
                hub_m: None,
                rd_m: Some(160.0),
            },
            TurbineInput {
                hub_m: None,
                rd_m: Some(160.0),
            },
        ],
        false,
        true,
    )
    .await;
    assert_eq!(
        props.rotor_diameter_mm,
        json!(160000),
        "single value when all turbines match"
    );

    let props = fetch_layout_props(
        &app,
        &auth,
        project_id,
        &[
            TurbineInput {
                hub_m: None,
                rd_m: Some(150.0),
            },
            TurbineInput {
                hub_m: None,
                rd_m: Some(160.0),
            },
            TurbineInput {
                hub_m: None,
                rd_m: Some(170.0),
            },
        ],
        false,
        true,
    )
    .await;
    assert_eq!(
        props.rotor_diameter_mm,
        json!("various"),
        "various when turbines differ"
    );

    let props = fetch_layout_props(
        &app,
        &auth,
        project_id,
        &[
            TurbineInput {
                hub_m: None,
                rd_m: Some(160.0),
            },
            TurbineInput {
                hub_m: None,
                rd_m: None,
            },
            TurbineInput {
                hub_m: None,
                rd_m: None,
            },
        ],
        false,
        true,
    )
    .await;
    assert_eq!(
        props.rotor_diameter_mm,
        json!("various"),
        "various when some turbines have no value"
    );

    let props = fetch_layout_props(
        &app,
        &auth,
        project_id,
        &[
            TurbineInput {
                hub_m: None,
                rd_m: Some(160.0),
            },
            TurbineInput {
                hub_m: None,
                rd_m: Some(160.0),
            },
            TurbineInput {
                hub_m: None,
                rd_m: Some(160.0),
            },
        ],
        false,
        false,
    )
    .await;
    assert_eq!(
        props.rotor_diameter_mm,
        json!(null),
        "null when field not mapped"
    );
}
