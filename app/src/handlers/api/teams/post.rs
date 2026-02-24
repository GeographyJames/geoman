use actix_web::{HttpResponse, web};

use crate::{
    app::{
        handlers::api::{ApiError, error::ToApiErr},
        session_state::{TypedSession, user_id},
    },
    domain::dtos::TeamInputDTO,
    postgres::PostgresRepo,
};

use super::TeamInputPayload;

#[tracing::instrument(skip(repo, session, payload))]
pub async fn post_team(
    repo: web::Data<PostgresRepo>,
    session: TypedSession,
    payload: web::Json<TeamInputPayload>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = user_id(&session)?;
    let admin = repo
        .is_admin(user_id)
        .await
        .map_err(|e| e.api_err("failed to query database"))?;
    if !admin {
        return Err(ApiError::AdminPermissionRequired.into());
    }
    let dto: TeamInputDTO = payload.into_inner().try_into()?;

    let id = repo
        .insert(&dto)
        .await
        .map_err(|e| e.api_err("failed to add team to database"))?;
    Ok(HttpResponse::Ok().json(id))
}
