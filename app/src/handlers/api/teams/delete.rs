use actix_web::{HttpResponse, web};

use crate::{
    app::{
        handlers::api::{ApiError, error::ToApiErr},
        session_state::{TypedSession, user_id},
    },
    domain::{dtos::Id, entities::Team},
    postgres::PostgresRepo,
};

#[tracing::instrument(skip(repo, team_id, session))]
pub async fn delete_team(
    repo: web::Data<PostgresRepo>,
    team_id: web::Path<Id>,
    session: TypedSession,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = user_id(&session)?;
    let admin = repo
        .is_admin(user_id)
        .await
        .map_err(|e| e.api_err("failed to query database"))?;
    if !admin {
        return Err(ApiError::AdminPermissionRequired.into());
    }

    repo.delete::<Team, _>(&team_id.into_inner())
        .await
        .map_err(|e| e.api_err("failed to delete team"))?;
    Ok(HttpResponse::Ok().finish())
}
