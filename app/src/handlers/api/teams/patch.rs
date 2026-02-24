use actix_web::{HttpResponse, patch, web};
use domain::{BusinessUnitId, TeamId};
use serde::{Deserialize, Serialize};

use crate::{AuthenticatedUser, errors::ApiError, postgres::PostgresRepo};

#[derive(Serialize, Deserialize)]
pub struct TeamUpdatePayload {
    pub business_unit: Option<BusinessUnitId>,
    pub name: Option<String>,
}

#[patch("/{team_id}")]
#[tracing::instrument(skip(repo, body, user, id))]
pub async fn patch_team(
    repo: web::Data<PostgresRepo>,
    body: web::Json<TeamUpdatePayload>,
    user: web::ReqData<AuthenticatedUser>,
    id: web::Path<TeamId>,
) -> Result<HttpResponse, ApiError> {
    if !user.admin {
        return Err(ApiError::AdminOnly);
    }
    repo.update(&(body.into_inner(), id.into_inner())).await?;
    Ok(HttpResponse::NoContent().finish())
}
