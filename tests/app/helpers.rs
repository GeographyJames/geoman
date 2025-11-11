use geoman::domain::FeatureId;
use reqwest::Response;
use serde::de::DeserializeOwned;

pub fn assert_ok(response: &reqwest::Response) {
    assert_eq!(
        response.status().as_u16(),
        200,
        "Expected 200 OK but got {}",
        response.status()
    )
}

pub fn assert_status(response: &reqwest::Response, expected_status: u16) {
    assert_eq!(
        response.status().as_u16(),
        expected_status,
        "Expected status {} but got {}",
        expected_status,
        response.status()
    )
}

/// Handles a response by returning the specified Json for successful responses or elegantly handling error cases or cases where the response body is not as expected
pub async fn handle_json_response<T: DeserializeOwned>(
    response: Response,
) -> Result<T, anyhow::Error> {
    let status = response.status();

    if status.is_success() {
        let json: T = response.json().await.map_err(|e| {
            anyhow::anyhow!(
                "Failed to deserialize successful {} response: {}",
                status,
                e
            )
        })?;
        return Ok(json);
    }

    let error = response
        .text()
        .await
        .unwrap_or_else(|_| "no response body".to_string());

    Err(anyhow::anyhow!(
        "Unsuccessful response status: {}\nbody:\n{}",
        status,
        error,
    ))
}

/// Asserts a GeoJson feature matches required criteria
pub fn check_feature<P: DeserializeOwned>(
    feature: &geojson::Feature,
    feature_id: Option<FeatureId>,
) {
    // Verify the feature has geometry
    assert!(feature.geometry.is_some(), "feature has no geometry");

    // Verify the feature has links
    assert!(
        feature
            .foreign_members
            .as_ref()
            .expect("no foreign members")
            .contains_key("links"),
        "feature has no links"
    );

    // Verify the feature has id that matches the expected feature_id
    let id = feature.id.as_ref().expect("feature has no id");

    match id {
        geojson::feature::Id::Number(number) => {
            let id_value: i32 = number
                .as_i64()
                .expect("feature id is not a valid i64")
                .try_into()
                .expect("feature id is not valid i32");

            if let Some(id) = feature_id {
                assert_eq!(id_value, id.0, "feature id does not match");
            }
        }
        geojson::feature::Id::String(_) => panic!("feature id is a string, expected number"),
    }

    // Verify the feature has properties
    let mut properties = feature
        .properties
        .as_ref()
        .expect("feature has no properties")
        .clone();

    // Verify properties has string field 'name'
    let name = properties
        .remove("name")
        .expect("properties has no name field");
    assert!(name.is_string(), "name field is not string");

    // Verify properties has a boolean field 'is_primary'
    let is_primary = properties
        .remove("is_primary")
        .expect("properties has no is_primary field");
    assert!(is_primary.is_boolean(), "is_primary is not a boolean");

    // Verify properties remaingin fields match type P
    let _properties_struct: P = serde_json::from_value(serde_json::Value::Object(properties))
        .expect("failed to deserialize properties to properties struct");
}
