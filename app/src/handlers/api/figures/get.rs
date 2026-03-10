use actix_web::{
    get,
    web::{self, Json},
};
use domain::{FigureId, ProjectId, figure::FigureOutputDTO};
use serde::Deserialize;

use crate::{errors::ApiError, postgres::PostgresRepo};

#[derive(Deserialize, Debug)]
pub struct GetFiguresQueryParams {
    project_id: ProjectId,
}

#[get("")]
#[tracing::instrument(skip(repo, params))]
pub async fn get_figures(
    repo: web::Data<PostgresRepo>,
    params: web::Query<GetFiguresQueryParams>,
) -> Result<Json<Vec<FigureOutputDTO>>, ApiError> {
    let figures = repo.get_figures_for_project(params.project_id).await?;
    Ok(Json(figures))
}

#[get("/{id}")]
#[tracing::instrument(skip(repo))]
pub async fn get_figure(
    repo: web::Data<PostgresRepo>,
    figure_id: web::Path<FigureId>,
) -> Result<Json<FigureOutputDTO>, ApiError> {
    let figure = repo.get_figure(figure_id.into_inner()).await?;
    Ok(Json(figure))
}
