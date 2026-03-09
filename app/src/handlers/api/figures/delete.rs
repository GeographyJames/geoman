use actix_web::{HttpResponse, web};

use crate::{
    app::handlers::api::ApiError,
    domain::{dtos::Id, entities::Figure},
    postgres::PostgresRepo,
};

#[tracing::instrument(skip(repo))]
pub async fn delete_figure(
    repo: web::Data<PostgresRepo>,
    figure_id: web::Path<Id>,
) -> Result<HttpResponse, actix_web::Error> {
    repo.delete::<Figure, _>(&figure_id.into_inner())
        .await
        .map_err(|e| ApiError::Repository {
            source: e,
            message: "failed to delete figure".into(),
        })?;
    Ok(HttpResponse::Ok().finish())
}
