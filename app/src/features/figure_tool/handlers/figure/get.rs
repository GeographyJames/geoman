use actix_web::{HttpResponse, web};

use serde::Deserialize;

use crate::{
    app::{
        configuration::Settings,
        features::figure_tool::{
            dtos::figure::FigureOutputDTO,
            ids::{FigureId, ProjectId},
            qgis_builder::{PrintResolution, generate_project},
        },
        handlers::api::ApiError,
        session_state::{TypedSession, user_id},
    },
    domain::dtos::UserOutputDTO,
    postgres::PostgresRepo,
    qgis::layer::{PgConfig, SslMode},
};

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    project: ProjectId,
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
    figure_id: web::Path<FigureId>,
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
