use actix_web::{HttpResponse, patch, web};

use crate::{
    AuthenticatedUser, errors::ApiError,
    features::figure_tool::{handlers::figure::FigurePayload, ids::FigureId},
    postgres::PostgresRepo,
};

#[patch("/{figure_id}")]
#[tracing::instrument(skip(repo, body, user, figure_id))]
pub async fn patch_figure(
    repo: web::Data<PostgresRepo>,
    body: web::Json<FigurePayload>,
    user: web::ReqData<AuthenticatedUser>,
    figure_id: web::Path<FigureId>,
) -> Result<HttpResponse, ApiError> {
    let user = user.into_inner();
    repo.update(&(body.into_inner(), figure_id.into_inner(), user.id))
        .await?;
    Ok(HttpResponse::NoContent().finish())
}
