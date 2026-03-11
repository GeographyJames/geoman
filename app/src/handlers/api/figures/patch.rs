use actix_web::{HttpResponse, patch, web};
use domain::FigureId;

use crate::{
    AuthenticatedUser, config::QgisServerSettings, errors::ApiError,
    handlers::api::figures::FigurePayload, postgres::PostgresRepo,
};

#[patch("/{id}")]
#[tracing::instrument(skip(repo, payload, user, qgis_server))]
pub async fn patch_figure(
    repo: web::Data<PostgresRepo>,
    user: web::ReqData<AuthenticatedUser>,
    figure_id: web::Path<FigureId>,
    payload: web::Json<FigurePayload>,
    qgis_server: web::Data<QgisServerSettings>,
) -> Result<HttpResponse, ApiError> {
    let user_id = user.id;
    let input_dto = payload
        .into_inner()
        .into_input_dto(user_id, Some(qgis_server.figure_config.clone()))
        .map_err(ApiError::FigureValidation)?;
    repo.update(&(&input_dto, figure_id.into_inner(), user_id))
        .await?;
    Ok(HttpResponse::NoContent().finish())
}
