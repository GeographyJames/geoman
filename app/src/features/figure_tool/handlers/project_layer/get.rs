use actix_web::{HttpResponse, web};
use serde::Deserialize;

use crate::{
    app::{
        features::figure_tool::{dtos::ProjectLayerOutputDTO, ids::ProjectId},
        handlers::api::ApiError,
    },
    postgres::PostgresRepo,
};

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    project: ProjectId,
}

#[tracing::instrument(skip(repo))]
pub async fn get_project_layers(
    project_id: web::Query<QueryParams>,
    repo: web::Data<PostgresRepo>,
) -> Result<HttpResponse, actix_web::Error> {
    let project_id = project_id.into_inner().project;

    let layers: Vec<ProjectLayerOutputDTO> = repo
        .select_all_for_project(&project_id)
        .await
        .map_err(|e| ApiError::Repository {
            source: e,
            message: "failed to retrieve project layers from database".into(),
        })?;

    Ok(HttpResponse::Ok().json(layers))
}
