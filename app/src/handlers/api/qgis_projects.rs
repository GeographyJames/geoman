use actix_web::{HttpResponse, get, web};

use crate::{errors::ApiError, postgres::PostgresRepo};

#[get("/{name}")]
#[tracing::instrument(skip(repo))]
pub async fn get_qgis_project(
    repo: web::Data<PostgresRepo>,
    name: web::Path<String>,
) -> Result<HttpResponse, ApiError> {
    let content = repo
        .get_qgis_project_content(&name)
        .await
        .map_err(|e| ApiError::Unexpected(anyhow::anyhow!("failed to fetch qgis project: {}", e)))?;

    match content {
        Some(bytes) => Ok(HttpResponse::Ok()
            .content_type("application/octet-stream")
            .body(bytes)),
        None => Err(ApiError::NotFound),
    }
}
