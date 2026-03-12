use actix_web::{
    post,
    web::{self, Json},
};

use crate::{
    AuthenticatedUser, config::QgisServerSettings, errors::ApiError,
    features::figure_tool::ids::FigureId, postgres::PostgresRepo,
};

use super::FigurePayload;

#[tracing::instrument(skip(repo, user, payload, qgis_config))]
#[post("")]
pub async fn post_figure(
    repo: web::Data<PostgresRepo>,
    user: web::ReqData<AuthenticatedUser>,
    payload: web::Json<FigurePayload>,
    qgis_config: web::Data<QgisServerSettings>,
) -> Result<Json<FigureId>, ApiError> {
    let payload = payload.into_inner();

    let input_dto = payload
        .into_input_dto(user.id, Some(qgis_config.figure_config.clone()))
        .map_err(ApiError::Validation)?;

    let figure_id = repo.insert(&input_dto).await?;
    Ok(Json(figure_id))
}
