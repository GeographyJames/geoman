use actix_web::{HttpResponse, delete, web};
use domain::TeamId;

use crate::{AuthenticatedUser, errors::ApiError, postgres::PostgresRepo};

#[delete("/{team_id}")]
#[tracing::instrument(skip(repo, user, id))]
pub async fn delete_team(
    repo: web::Data<PostgresRepo>,
    user: web::ReqData<AuthenticatedUser>,
    id: web::Path<TeamId>,
) -> Result<HttpResponse, ApiError> {
    if !user.admin {
        return Err(ApiError::AdminOnly);
    }
    repo.delete_team(id.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}
