use actix_multipart::form::{MultipartForm, tempfile::TempFile};
use actix_web::{post, web::Json};
use anyhow::Context;
use gdal::{vsi, vector::LayerAccess};
use geo::virtual_shapefile::{ShapefileError, get_epsg_from_prj};
use serde::{Deserialize, Serialize};
use std::io::Read;
use uuid::Uuid;

use crate::errors::ApiError;

#[derive(Deserialize)]
pub struct EpsgRequest {
    pub prj: String,
}

#[derive(Serialize)]
pub struct EpsgResponse {
    pub srid: i32,
    pub name: Option<String>,
}

#[tracing::instrument(skip(payload))]
#[post("")]
pub async fn post_epsg(payload: Json<EpsgRequest>) -> Result<Json<EpsgResponse>, ApiError> {
    let crs = get_epsg_from_prj(&payload.prj)?;
    Ok(Json(EpsgResponse {
        srid: crs.srid,
        name: crs.name,
    }))
}

#[derive(MultipartForm)]
pub struct ShzEpsgPayload {
    pub shz: TempFile,
}

#[tracing::instrument(skip(payload))]
#[post("/shz")]
pub async fn post_epsg_from_shz(
    payload: MultipartForm<ShzEpsgPayload>,
) -> Result<Json<EpsgResponse>, ApiError> {
    let mut shz = payload.into_inner().shz;
    let mut bytes = Vec::new();
    shz.file
        .read_to_end(&mut bytes)
        .context("failed to read shz file")
        .map_err(ShapefileError::UnexpectedError)?;
    let path = format!("/vsimem/{}.shz", Uuid::new_v4());
    vsi::create_mem_file(&path, bytes)
        .context("failed to create virtual shz file")
        .map_err(ShapefileError::UnexpectedError)?;
    let ds = gdal::Dataset::open(&path)
        .context("failed to open shz dataset")
        .map_err(ShapefileError::UnexpectedError)?;
    let layer = ds
        .layers()
        .next()
        .context("no layers in shz file")
        .map_err(ShapefileError::UnexpectedError)?;
    let srs = layer
        .spatial_ref()
        .context("no spatial reference")
        .map_err(ShapefileError::InvalidData)?;
    let srid = srs
        .auth_code()
        .context("failed to retrieve spatial ref auth code")
        .map_err(ShapefileError::InvalidData)?;
    let name = srs.name();
    let _ = vsi::unlink_mem_file(&path);
    Ok(Json(EpsgResponse { srid, name }))
}
