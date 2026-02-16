use app::{ErrorResponse, MockUserCredentials};
use gdal::{
    Dataset,
    vector::{Geometry, Layer, LayerOptions},
};
use geo::virtual_shapefile::ShapefileData;
use rand::RngExt;
use reqwest::{
    RequestBuilder, Response,
    multipart::{Form, Part},
};
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

pub fn generate_random_bng_point_wkt() -> (f32, f32, String) {
    let mut rng = rand::rng();
    let easting: f32 = rng.random_range(0.0..700_000.);
    let northing: f32 = rng.random_range(0.0..1_300_000.);
    (
        easting,
        northing,
        format!("POINT({} {})", easting, northing),
    )
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
            Auth::_MockUserCredentials(user) => req.header(
                "X-Test-User",
                json!(MockUserCredentials::User(user.clone())).to_string(),
            ),
        }
    } else {
        req
    }
}

pub fn create_gdal_multipolygon_bng() -> Geometry {
    let mut rng = rand::rng();
    let x1 = rng.random_range(0..699_998);
    let x2 = rng.random_range(x1..699_999);
    let y1 = rng.random_range(0..1_299_998);
    let y2 = rng.random_range(y1..1_299_999);

    let geom = Geometry::from_wkt(&format!(
        "MULTIPOLYGON((({x1} {y1}, {x2} {y1},  {x2} {y2}, {x1} {y2}, {x1} {y1})))"
    ))
    .expect("failed to create polygon from wkt");
    assert!(geom.is_valid(), "invalid geometry");
    assert!(!geom.is_empty(), "geometry empty");
    geom
}

pub fn _create_gdal_multipolygon_wgs84() -> Geometry {
    let geom = Geometry::from_wkt("MULTIPOLYGON(((-3 52, -2 54, 0 52, -3 52)))")
        .expect("failed to create wgs 84 polygon from wkt");
    assert!(geom.is_valid(), "invalid geometry");
    assert!(!geom.is_empty(), "geometry empty");
    geom
}
pub fn create_shapefile_dataset() -> (Dataset, String) {
    let filename = format!("/vsimem/{}.shp", uuid::Uuid::new_v4());
    let dataset = gdal::DriverManager::get_driver_by_name("ESRI Shapefile")
        .expect("failed to get shapefile driver")
        .create_vector_only(&filename)
        .expect("failed to create layer");
    (dataset, filename)
}

pub fn add_layer(dataset: &mut Dataset, ty: u32, crs: u32) -> Layer<'_> {
    dataset
        .create_layer(LayerOptions {
            name: "test",
            options: None,
            ty,
            srs: Some(&gdal::spatial_ref::SpatialRef::from_epsg(crs).expect("failed to get rsr")),
        })
        .expect("failed to create layer")
}

pub fn dataset_to_shapefile_data(mut dataset: Dataset, filename: &str) -> ShapefileData {
    dataset.flush_cache().expect("failed to flush cache");
    dataset.close().expect("failed to close dataset");
    ShapefileData::try_from_gdal_vsi_mem_file(filename).expect("failed to create shapefile data")
}

pub fn create_gdal_point_bng() -> Geometry {
    let geom = Geometry::from_wkt("POINT(123456 654321)").expect("failed to create poient");
    assert!(geom.is_valid(), "invalid geometry");
    assert!(!geom.is_empty(), "geometry empty");
    geom
}

pub fn create_gdal_point_wgs84() -> Geometry {
    let geom = Geometry::from_wkt("POINT(-3 52)").expect("failed to create poient");
    assert!(geom.is_valid(), "invalid geometry");
    assert!(!geom.is_empty(), "geometry empty");
    geom
}

pub fn add_shz_to_form(shz_bytes: Vec<u8>, form: Form) -> Form {
    form.part(
        "shz",
        Part::bytes(shz_bytes)
            .file_name("shapefile.shz")
            .mime_str("application/octet-stream")
            .expect("failed to add shz part"),
    )
}

pub fn add_shapefile_to_form(filename: &str, data: ShapefileData, form: Form) -> Form {
    let mime = "application/octet-stream";
    form.part(
        "shp",
        Part::bytes(data.shp)
            .file_name(format!("{filename}.shp"))
            .mime_str(mime)
            .expect("failed to shp part"),
    )
    .part(
        "shx",
        Part::bytes(data.shx)
            .file_name(format!("{filename}.shx"))
            .mime_str(mime)
            .expect("failed to add shx part"),
    )
    .part(
        "prj",
        Part::bytes(data.prj)
            .file_name(format!("{filename}.prj"))
            .mime_str(mime)
            .expect("failed to add prj part"),
    )
    .part(
        "dbf",
        Part::bytes(data.dbf)
            .file_name(format!("{filename}.dbf"))
            .mime_str(mime)
            .expect("failed to add dbf part"),
    )
}
