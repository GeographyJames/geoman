use actix_web::{HttpResponse, get, web};

use crate::{
    config::{DatabaseSettings, QgisServerSettings},
    errors::ApiError,
    features::figure_tool::{
        PrintResolution,
        dtos::FigureOutputDTO,
        ids::FigureId,
        qgis_builder::generate_project,
    },
    postgres::PostgresRepo,
};
use qgis::layer::{PgConfig, SslMode};

#[get("/{figure_id}/qgz")]
#[tracing::instrument(skip(repo, qgis_server, db_settings))]
pub async fn get_figure_qgz(
    repo: web::Data<PostgresRepo>,
    path: web::Path<FigureId>,
    qgis_server: web::Data<QgisServerSettings>,
    db_settings: web::Data<DatabaseSettings>,
) -> Result<HttpResponse, ApiError> {
    let figure_id = path.into_inner();
    let mut figure: FigureOutputDTO = repo
        .select_one::<FigureOutputDTO, _>(figure_id)
        .await?
        .ok_or(ApiError::NotFound)?;

    figure.set_basemap_urls_to_alt_urls();

    let filename = figure.filename_without_id("qgz");
    let qgis_project = generate_project(
        figure,
        Some(&qgis_server.figure_config),
        &PrintResolution::High,
        false,
        PgConfig {
            db_name: db_settings.database_name.clone(),
            port: db_settings.port,
            host: db_settings.host.clone(),
            sslmode: SslMode::from(db_settings.require_ssl),
        },
        None,
    )
    .map_err(|e| ApiError::Unexpected(e.context("failed to create qgis project")))?;

    Ok(HttpResponse::Ok()
        .content_type("application/octet-stream")
        .insert_header((
            "Content-Disposition",
            format!("attachment; filename=\"{}\"", filename),
        ))
        .body(qgis_project.content))
}
