use ogc::types::Feature;
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

pub async fn check_error_response(
    response: reqwest::Response,
    expected_status: u16,
    message: &str,
) {
    assert_status(&response, expected_status);
    assert_eq!(
        response.text().await.expect("response has no body"),
        message
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

pub fn check_feature<P: DeserializeOwned>(feature: &Feature) {
    // Verify the feature has properties
    let mut properties = feature.properties.clone();

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
