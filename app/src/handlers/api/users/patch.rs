use actix_web::{
    HttpResponse, patch,
    web::{self},
};
use domain::{TeamId, UserId};
use serde::{Deserialize, Serialize};

use crate::{AuthenticatedUser, errors::ApiError, postgres::PostgresRepo};

#[derive(Serialize, Deserialize)]
pub struct PatchUserPayload {
    pub team_id: Option<TeamId>,
    pub admin: Option<bool>,
}

#[patch("/{user_id}")]
#[tracing::instrument(skip(repo, body, user, id))]
pub async fn patch_user(
    repo: web::Data<PostgresRepo>,
    body: web::Json<PatchUserPayload>,
    user: web::ReqData<AuthenticatedUser>,
    id: web::Path<UserId>,
) -> Result<HttpResponse, ApiError> {
    if !user.admin {
        return Err(ApiError::AdminOnly);
    }
    repo.update(&(body.into_inner(), id.into_inner())).await?;
    Ok(HttpResponse::NoContent().finish())
}
