use actix_web::{HttpResponse, web};

use crate::{
    app::{
        configuration::Settings,
        handlers::api::{ApiError, figures::FigurePayload},
        session_state::{TypedSession, user_id},
    },
    postgres::PostgresRepo,
};

#[tracing::instrument(skip(repo, session, payload, config))]
pub async fn post_figure(
    repo: web::Data<PostgresRepo>,
    session: TypedSession,
    payload: web::Json<FigurePayload>,
    config: web::Data<Settings>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = user_id(&session)?;
    let payload = payload.into_inner();

    let input_dto = payload
        .into_input_dto(user_id, Some(config.qgis_server.figure_config.clone()))
        .map_err(ApiError::Validation)?;

    let figure_id = repo
        .insert(&input_dto)
        .await
        .map_err(|e| ApiError::Repository {
            source: (e),
            message: ("failed to add figure to database".into()),
        })?;
    Ok(HttpResponse::Ok().json(figure_id))
}
