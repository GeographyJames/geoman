use actix_web::{HttpResponse, get, web};

use crate::{
    errors::ApiError,
    features::figure_tool::dtos::QgisProjectName,
    postgres::PostgresRepo,
};
use crate::features::figure_tool::db::qgis_project::select_qgis_project;

#[get("/{name}")]
#[tracing::instrument(skip(repo))]
pub async fn get_qgis_project(
    repo: web::Data<PostgresRepo>,
    name: web::Path<String>,
) -> Result<HttpResponse, ApiError> {
    let project_name = QgisProjectName(name.into_inner());
    let mut conn = repo.db_pool.acquire().await.map_err(|e| {
        ApiError::Unexpected(anyhow::anyhow!("failed to acquire db connection: {}", e))
    })?;
    let project = select_qgis_project(&mut conn, &project_name).await?;
    Ok(HttpResponse::Ok()
        .content_type("application/octet-stream")
        .append_header((
            "Content-Disposition",
            format!("attachment; filename=\"{}.qgz\"", project_name.0),
        ))
        .body(project.content))
}
