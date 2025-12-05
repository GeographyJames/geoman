use actix_web::{HttpResponse, http::StatusCode, patch, web};
use domain::KeyId;

use crate::{handlers::ApiError, postgres::PostgresRepo, types::AuthenticatedUser};

#[patch("/{id}/renew")]
#[tracing::instrument(skip(repo, id, user))]
pub async fn renew_api_key(
    repo: web::Data<PostgresRepo>,
    id: web::Path<KeyId>,
    user: web::ReqData<AuthenticatedUser>,
) -> Result<HttpResponse, ApiError> {
    repo.renew_api_key(id.into_inner(), user.id).await?;
    Ok(HttpResponse::new(StatusCode::NO_CONTENT))
}
