use actix_web::{HttpResponse, http::StatusCode, patch, web};
use clerk_rs::validators::authorizer::ClerkJwt;
use domain::KeyId;

use crate::{handlers::ApiError, postgres::PostgresRepo};

#[patch("/{id}/revoke")]
#[tracing::instrument(skip(repo, id))]
pub async fn revoke_api_key(
    repo: web::Data<PostgresRepo>,
    id: web::Path<KeyId>,
    user: web::ReqData<ClerkJwt>,
) -> Result<HttpResponse, ApiError> {
    println!("{}", user.sub);
    repo.revoke_api_key(id.into_inner(), &user.into_inner())
        .await?;
    Ok(HttpResponse::new(StatusCode::NO_CONTENT))
}
