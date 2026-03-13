use actix_web::{get, web};
use actix_web::web::Json;
use serde::Deserialize;

use crate::{
    errors::ApiError,
    features::figure_tool::dtos::ProjectLayerOutputDTO,
    postgres::PostgresRepo,
};
use domain::ProjectId;

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    project: ProjectId,
}

#[get("")]
#[tracing::instrument(skip(repo))]
pub async fn get_project_layers(
    params: web::Query<QueryParams>,
    repo: web::Data<PostgresRepo>,
) -> Result<Json<Vec<ProjectLayerOutputDTO>>, ApiError> {
    let (layers, _) = repo
        .select_all_with_params::<ProjectLayerOutputDTO>(params.into_inner().project)
        .await?;
    Ok(Json(layers))
}
