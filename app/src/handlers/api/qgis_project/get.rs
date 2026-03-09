use actix_web::{HttpResponse, web};

use crate::{
    app::handlers::api::ApiError, domain::dtos::QgisProjectName, postgres::PostgresRepo,
    qgis::project::QgisProject,
};

#[tracing::instrument(skip(repo))]
pub async fn get_qgis_project(
    repo: web::Data<PostgresRepo>,
    id: web::Path<String>,
) -> Result<HttpResponse, actix_web::Error> {
    let project_name = QgisProjectName(id.into_inner());
    let project: QgisProject =
        repo.select(&project_name)
            .await
            .map_err(|e| ApiError::Repository {
                source: e,
                message: "failed to retrieve qgis project from database".into(),
            })?;
    Ok(HttpResponse::Ok()
        .content_type("application/octet-stream")
        .append_header((
            "Content-Disposition",
            format!("attachment; filename=\"{}.qgz\"", project_name.0),
        ))
        .body(project.content))
}
