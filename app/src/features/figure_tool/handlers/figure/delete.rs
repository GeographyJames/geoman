use actix_web::{HttpResponse, web};

use crate::{
    app::{
        features::figure_tool::{entities::Figure, ids::FigureId},
        handlers::api::ApiError,
    },
    postgres::PostgresRepo,
};

#[tracing::instrument(skip(repo))]
pub async fn delete_figure(
    repo: web::Data<PostgresRepo>,
    figure_id: web::Path<FigureId>,
) -> Result<HttpResponse, actix_web::Error> {
    repo.delete::<Figure, _>(&figure_id.into_inner())
        .await
        .map_err(|e| ApiError::Repository {
            source: e,
            message: "failed to delete figure".into(),
        })?;
    Ok(HttpResponse::Ok().finish())
}
