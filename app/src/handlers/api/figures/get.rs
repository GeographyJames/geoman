use actix_web::{HttpResponse, web};

use serde::Deserialize;

use crate::{
    app::{
        configuration::Settings,
        handlers::api::ApiError,
        session_state::{TypedSession, user_id},
    },
    domain::dtos::{FigureOutputDTO, Id, UserOutputDTO},
    postgres::PostgresRepo,
    qgis::{
        figure::{PrintResolution, generate_project},
        layer::{PgConfig, SslMode},
    },
};

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    project: Id,
}

#[tracing::instrument(skip(repo))]
pub async fn get_figures(
    repo: web::Data<PostgresRepo>,
    params: web::Query<QueryParams>,
) -> Result<HttpResponse, actix_web::Error> {
    let figure: Vec<FigureOutputDTO> =
        repo.select_all_for_project(&params.project)
            .await
            .map_err(|e| ApiError::Repository {
                source: e,
                message: "failed to retrieve figures from database".into(),
            })?;
    Ok(HttpResponse::Ok().json(figure))
}

#[tracing::instrument(skip(repo))]
pub async fn get_figure(
    repo: web::Data<PostgresRepo>,
    figure_id: web::Path<Id>,
) -> Result<HttpResponse, actix_web::Error> {
    let figure: FigureOutputDTO =
        repo.select(&figure_id.into_inner())
            .await
            .map_err(|e| ApiError::Repository {
                source: e,
                message: "failed to retrieve figure from database".into(),
            })?;
    Ok(HttpResponse::Ok().json(figure))
}

#[tracing::instrument(skip(repo, config, session))]
pub async fn get_figure_qgis_project(
    repo: web::Data<PostgresRepo>,
    figure_id: web::Path<Id>,
    config: web::Data<Settings>,
    session: TypedSession,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = user_id(&session)?;
    let user: UserOutputDTO = repo
        .select(&user_id)
        .await
        .map_err(|e| ApiError::Repository {
            source: e,
            message: "failed to retrieve user from database".into(),
        })?;
    let mut figure: FigureOutputDTO =
        repo.select(&figure_id.into_inner())
            .await
            .map_err(|e| ApiError::Repository {
                source: e,
                message: "failed to retrieve figure from database".into(),
            })?;

    // Switch base map urls for download urls if they exist
    figure.set_basemap_urls_to_alt_urls();

    let filename = figure.filename_without_id("qgz");
    let qgis_project = generate_project(
        figure,
        Some(&config.qgis_server.figure_config),
        &PrintResolution::High,
        false,
        PgConfig {
            db_name: config
                .database
                .connection_pool_name
                .clone()
                .unwrap_or(config.database.database_name.clone()),
            port: config
                .database
                .connection_pool_port
                .unwrap_or(config.database.port),
            host: config.database.host.clone(),
            sslmode: SslMode::from(config.database.require_ssl),
        },
        user.qgis_pg_authcfg_id.clone(),
    )
    .map_err(|e| {
        ApiError::Unexpected(anyhow::anyhow!("failed to create qgis project file: {}", e))
    })?;

    Ok(HttpResponse::Ok()
        .content_type("application/octet-stream")
        .append_header((
            "Content-Disposition",
            format!("attachment; filename=\"{}\"", filename),
        ))
        .body(qgis_project.content))
}
