use actix_web::{HttpResponse, web};

use crate::{
    app::{
        handlers::api::{ApiError, error::ToApiErr},
        session_state::{TypedSession, user_id},
    },
    domain::dtos::{Id, TeamInputDTO},
    postgres::PostgresRepo,
};

use super::TeamInputPayload;

#[tracing::instrument(skip(repo, id, session, payload))]
pub async fn put_team(
    repo: web::Data<PostgresRepo>,
    session: TypedSession,
    payload: web::Json<TeamInputPayload>,
    id: web::Path<Id>,
) -> Result<HttpResponse, actix_web::Error> {
    let current_user = user_id(&session)?;
    let admin = repo
        .is_admin(current_user)
        .await
        .map_err(|e| e.api_err("unable to query database to check admin privalidges"))?;

    if !admin {
        return Err(ApiError::AdminPermissionRequired.into());
    }
    let dto: TeamInputDTO = payload.into_inner().try_into()?;

    repo.update(&dto, &id.into_inner())
        .await
        .map_err(|e| e.api_err("failed to add team to database"))?;

    Ok(HttpResponse::Ok().finish())
}
