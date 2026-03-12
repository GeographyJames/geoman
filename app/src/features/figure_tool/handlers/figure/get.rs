use actix_web::{
    get,
    web::{self, Json},
};
use domain::ProjectId;
use serde::Deserialize;

use crate::{
    errors::ApiError,
    features::figure_tool::{dtos::FigureOutputDTO, ids::FigureId},
    postgres::PostgresRepo,
};

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    project: ProjectId,
}

#[get("")]
#[tracing::instrument(skip(repo))]
pub async fn get_figures(
    repo: web::Data<PostgresRepo>,
    params: web::Query<QueryParams>,
) -> Result<Json<Vec<FigureOutputDTO>>, ApiError> {
    let (figures, _): (Vec<FigureOutputDTO>, _) = repo
        .select_all_with_params(params.into_inner().project)
        .await?;
    Ok(Json(figures))
}

#[get("/{figure_id}")]
#[tracing::instrument(skip(repo))]
pub async fn get_figure(
    repo: web::Data<PostgresRepo>,
    figure_id: web::Path<FigureId>,
) -> Result<Json<FigureOutputDTO>, ApiError> {
    let figure = repo
        .select_one::<FigureOutputDTO, _>(figure_id.into_inner())
        .await?
        .ok_or(ApiError::NotFound)?;
    Ok(Json(figure))
}
