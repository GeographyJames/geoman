use actix_web::{HttpResponse, web};

use crate::{
    app::{
        configuration::Settings,
        features::figure_tool::{handlers::figure::FigurePayload, ids::FigureId},
        handlers::api::ApiError,
        session_state::{TypedSession, user_id},
    },
    postgres::PostgresRepo,
};

#[tracing::instrument(skip(repo, payload, figure_id, session, config))]
pub async fn put_figure(
    repo: web::Data<PostgresRepo>,
    payload: web::Json<FigurePayload>,
    figure_id: web::Path<FigureId>,
    session: TypedSession,
    config: web::Data<Settings>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = user_id(&session)?;
    let payload = payload.into_inner();

    let input_dto = payload
        .into_input_dto(user_id, Some(config.qgis_server.figure_config.clone()))
        .map_err(ApiError::Validation)?;
    repo.update(&input_dto, &figure_id)
        .await
        .map_err(|e| ApiError::Repository {
            source: (e),
            message: ("failed to update figure in database".into()),
        })?;
    Ok(HttpResponse::Ok().finish())
}
