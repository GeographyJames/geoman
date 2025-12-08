use app::{ErrorResponse, MockUserCredentials};
use rand::Rng;
use reqwest::{RequestBuilder, Response};
use serde::de::DeserializeOwned;
use serde_json::json;

use crate::common::Auth;

/// Cheks response is 200
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
) -> ErrorResponse {
    assert_status(&response, expected_status);
    response
        .json()
        .await
        .expect("failed to deserialise response")
}

/// Handles a response by returning the specified JSON type for successful responses,
/// or pretty-printing useful error information for failure cases.
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

    // Read the body as text
    let body = response
        .text()
        .await
        .unwrap_or_else(|_| "no response body".to_string());

    // Try to parse it as JSON for pretty-printing
    let pretty_body = match serde_json::from_str::<serde_json::Value>(&body) {
        Ok(json_value) => {
            serde_json::to_string_pretty(&json_value).unwrap_or_else(|_| body.clone())
        }
        Err(_) => body.clone(), // not JSON, return raw text
    };

    Err(anyhow::anyhow!(
        "Unsuccessful response status: {}\nResponse Body:\n{}",
        status,
        pretty_body
    ))
}

pub fn generate_random_bng_point_ewkt() -> (f32, f32, String) {
    let mut rng = rand::rng();
    let easting: f32 = rng.random_range(0.0..700_000.);
    let northing: f32 = rng.random_range(0.0..1_300_000.);
    (easting, northing, generate_point(easting, northing, 27700))
}

pub fn generate_point(x: f32, y: f32, srid: u32) -> String {
    format!("SRID={};POINT({} {})", srid, x, y)
}

pub fn generate_random_wgs84_point_ewkt() -> (f32, f32, String) {
    let mut rng = rand::rng();
    let lat: f32 = rng.random_range(-90.0..90.);
    let long: f32 = rng.random_range(-180.0..180.);
    (long, lat, generate_point(long, lat, 4326))
}

pub fn auth_request(req: RequestBuilder, auth: Option<&Auth>) -> RequestBuilder {
    if let Some(auth) = auth {
        match auth {
            Auth::Key(key) => req.bearer_auth(key),
            Auth::Token(token) => req.bearer_auth(&token.0),
            Auth::MockToken(token) => req.header(
                "X-Test-User",
                json!(MockUserCredentials::Token(token.clone())).to_string(),
            ),
        }
    } else {
        req
    }
}
