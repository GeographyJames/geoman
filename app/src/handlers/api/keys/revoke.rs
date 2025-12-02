use actix_web::{HttpResponse, http::StatusCode, patch, web};
use domain::{KeyId, UserId};

use crate::{handlers::ApiError, postgres::PostgresRepo};

#[patch("/{id}/revoke")]
#[tracing::instrument(skip(repo, id, user_id))]
pub async fn revoke_api_key(
    repo: web::Data<PostgresRepo>,
    id: web::Path<KeyId>,
    user_id: web::ReqData<UserId>,
) -> Result<HttpResponse, ApiError> {
    repo.revoke_api_key(id.into_inner(), user_id.into_inner())
        .await?;
    Ok(HttpResponse::new(StatusCode::NO_CONTENT))
}
