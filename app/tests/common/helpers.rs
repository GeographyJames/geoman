use domain::ProjectFeature;
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

pub fn check_ogc_feature<P: DeserializeOwned>(ogc_feature: ogc::Feature) {
    let project_feature = ProjectFeature::try_from(ogc_feature)
        .expect("failed to convert ogc featuer to project feature");
    let _props: P = serde_json::from_value(serde_json::Value::Object(project_feature.properties))
        .expect("failed to deserialise feature properties to properties struct");
}

pub async fn check_conformance_declaration_response(response: Response) {
    assert_ok(&response);
    let _conformance: ogc::ConformanceDeclaration = handle_json_response(response)
        .await
        .expect("failed to retrieve conformance");
}

pub async fn check_landing_page_response(response: Response) {
    assert_ok(&response);
    let _landing_page: ogc::LandingPage = handle_json_response(response)
        .await
        .expect("failed to retrieve landing page");
}
