use actix_web::{
    post,
    web::{self, Json},
};
use domain::FigureId;

use crate::{
    AuthenticatedUser, config::QgisServerSettings, errors::ApiError,
    handlers::api::figures::FigurePayload, postgres::PostgresRepo,
};

#[post("")]
#[tracing::instrument(skip(repo, payload, qgis_server))]
pub async fn post_figure(
    repo: web::Data<PostgresRepo>,

    user: web::ReqData<AuthenticatedUser>,
    payload: web::Json<FigurePayload>,
    qgis_server: web::Data<QgisServerSettings>,
) -> Result<Json<FigureId>, ApiError> {
    let payload = payload.into_inner();
    let user_id = user.id;

    let input_dto = payload
        .into_input_dto(user_id, Some(qgis_server.figure_config.clone()))
        .map_err(ApiError::FigureValidation)?;

    let figure_id = repo.insert(&(&input_dto, user_id)).await?;

    Ok(Json(figure_id))
}
