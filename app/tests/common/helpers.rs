use app::ErrorResponse;
use rand::Rng;
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
) -> ErrorResponse {
    assert_status(&response, expected_status);
    response
        .json()
        .await
        .expect("failed to deserialise response")
}

/// Handles a response by returning the specified Json for successful responses or elegantly handling error cases or cases where the response body is not as expected
pub async fn handle_json_response<T: DeserializeOwned>(
    response: Response,
) -> Result<T, anyhow::Error> {
    let status = response.status();

    if status.is_success() {
        let json: T = response.json().await.map_err(|e| {
            anyhow::anyhow!(
                "Failed to deserialize successful {} response: {:?}",
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
