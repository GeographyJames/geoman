use actix_web::{post, web::Json};
use geo::virtual_shapefile::get_epsg_from_prj;
use serde::{Deserialize, Serialize};

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
