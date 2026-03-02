use app::constants::TURBINE_LAYOUTS_COLLECTION_ID;
use domain::ProjectCollectionId;

use crate::common::{
    AppBuilder, Auth,
    helpers::{TurbineInput, assert_ok, handle_json_response},
};

#[tokio::test]
async fn get_turbine_layout_geojson_works() {
    let app = AppBuilder::new().build().await;
    let auth = Auth::mock_session_token();
    let project_id = app.generate_project_id(Some(&auth)).await;

    let turbines = vec![
        TurbineInput {
            hub_m: Some(120.0),
            rd_m: Some(160.0),
        },
        TurbineInput {
            hub_m: Some(100.0),
            rd_m: Some(140.0),
        },
        TurbineInput {
            hub_m: None,
            rd_m: None,
        },
    ];
    let response = app
        .post_turbine_layout(&project_id, &turbines, true, true, true, Some(&auth))
        .await;
    assert_ok(&response);
    let layout_id = handle_json_response::<domain::LayoutId>(response)
        .await
        .expect("failed to parse layout id");

    let collection_slug = app
        .get_collection_slug(
            project_id,
            ProjectCollectionId(TURBINE_LAYOUTS_COLLECTION_ID),
        )
        .await;
    let project_slug = app.get_project_slug(project_id).await;

    let response = app
        .get_feature_geojson(
            Some(&auth),
            project_slug.as_ref(),
            collection_slug.as_ref(),
            layout_id.0,
        )
        .await;
    assert_ok(&response);

    let fc: geojson::FeatureCollection = handle_json_response(response)
        .await
        .expect("failed to parse feature collection");

    assert_eq!(fc.features.len(), 3, "expected 3 turbine features");

    for (i, feature) in fc.features.iter().enumerate() {
        let turbine_number = i as i64 + 1;

        // Each feature must be a Point in WGS84
        let geom = feature.geometry.as_ref().expect("feature has no geometry");
        assert!(
            matches!(geom.value, geojson::Value::Point(_)),
            "expected Point geometry"
        );
        if let geojson::Value::Point(coords) = &geom.value {
            assert!(
                coords[0] >= -180.0 && coords[0] <= 180.0,
                "longitude out of WGS84 range"
            );
            assert!(
                coords[1] >= -90.0 && coords[1] <= 90.0,
                "latitude out of WGS84 range"
            );
        }

        let props = feature
            .properties
            .as_ref()
            .expect("feature has no properties");
        assert_eq!(
            props["turbine_number"].as_i64(),
            Some(turbine_number),
            "turbine_number mismatch for feature {i}"
        );
    }

    // Check properties for turbines with values
    let props0 = fc.features[0].properties.as_ref().unwrap();
    assert_eq!(props0["hub_height_mm"].as_i64(), Some(120_000));
    assert_eq!(props0["rotor_diameter_mm"].as_i64(), Some(160_000));
    assert!(
        props0["x_storage_crs"].is_number(),
        "x_storage_crs should be a number"
    );
    assert!(
        props0["y_storage_crs"].is_number(),
        "y_storage_crs should be a number"
    );

    let props1 = fc.features[1].properties.as_ref().unwrap();
    assert_eq!(props1["hub_height_mm"].as_i64(), Some(100_000));
    assert_eq!(props1["rotor_diameter_mm"].as_i64(), Some(140_000));

    // Third turbine has no hub height or rotor diameter
    let props2 = fc.features[2].properties.as_ref().unwrap();
    assert!(props2["hub_height_mm"].is_null());
    assert!(props2["rotor_diameter_mm"].is_null());

    // FeatureCollection foreign members should include storage CRS info
    let fm = fc.foreign_members.as_ref().expect("no foreign members");
    assert!(
        fm["storage_crs_srid"].is_number(),
        "storage_crs_srid should be a number"
    );
    assert!(
        fm["storage_crs_name"].is_string(),
        "storage_crs_name should be a string"
    );
}
