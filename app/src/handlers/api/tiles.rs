use actix_web::{HttpResponse, get, web};
use serde::Deserialize;

use crate::{config::GeoserverSettings, errors::ApiError};

#[derive(Deserialize, Debug)]
pub struct TilePath {
    workspace: String,
    layer: String,
    z: u32,
    x: u32,
    y: u32,
}

#[get("/{workspace}/{layer}/{z}/{x}/{y}")]
#[tracing::instrument(skip(client, geoserver))]
pub async fn get_tile(
    path: web::Path<TilePath>,
    client: web::Data<reqwest::Client>,
    geoserver: web::Data<GeoserverSettings>,
) -> Result<HttpResponse, ApiError> {
    let url = format!(
        "{}gwc/service/tms/1.0.0/{}:{}@EPSG:900913@pbf/{}/{}/{}.pbf",
        geoserver.url, path.workspace, path.layer, path.z, path.x, path.y
    );

    let response = client
        .get(&url)
        // .basic_auth(&geoserver.username, Some(geoserver.password.expose_secret()))
        .send()
        .await
        .map_err(|e| anyhow::anyhow!(e))?;

    if !response.status().is_success() {
        tracing::warn!(
            geoserver_status = %response.status(),
            geoserver_url = %url,
            "GeoServer returned non-success status"
        );
        return Err(ApiError::NotFound);
    }

    let bytes = response.bytes().await.map_err(|e| anyhow::anyhow!(e))?;

    Ok(HttpResponse::Ok()
        .content_type("application/vnd.mapbox-vector-tile")
        .body(bytes))
}
