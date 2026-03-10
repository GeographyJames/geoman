use actix_web::{HttpResponse, delete, web};
use domain::FigureId;

use crate::{errors::ApiError, postgres::PostgresRepo};

#[delete("/{id}")]
#[tracing::instrument(skip(repo))]
pub async fn delete_figure(
    repo: web::Data<PostgresRepo>,
    figure_id: web::Path<FigureId>,
) -> Result<HttpResponse, ApiError> {
    repo.delete_figure(figure_id.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}
