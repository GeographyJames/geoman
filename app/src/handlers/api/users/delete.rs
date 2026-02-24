use actix_web::{HttpResponse, delete, web};
use domain::UserId;

use crate::{AuthenticatedUser, errors::ApiError, postgres::PostgresRepo};

#[delete("/{user_id}")]
#[tracing::instrument(skip(repo, user, id))]
pub async fn delete_user(
    repo: web::Data<PostgresRepo>,
    user: web::ReqData<AuthenticatedUser>,
    id: web::Path<UserId>,
) -> Result<HttpResponse, ApiError> {
    if !user.admin {
        return Err(ApiError::AdminOnly);
    }
    let target_id = id.into_inner();
    if target_id == user.id {
        return Err(ApiError::AdminOnly);
    }
    repo.delete_user(target_id).await?;
    Ok(HttpResponse::NoContent().finish())
}
