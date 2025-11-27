use actix_web::{HttpResponse, post, web};
use anyhow::Context;
use clerk_rs::validators::authorizer::ClerkJwt;

use crate::{handlers::ApiError, postgres::PostgresRepo};

#[post("")]
#[tracing::instrument(skip(jwt, repo))]
pub async fn generate_api_key(
    jwt: web::ReqData<ClerkJwt>,
    repo: web::Data<PostgresRepo>,
) -> Result<HttpResponse, ApiError> {
    let clerk_user_id = &jwt.sub;
    let user = sqlx::query_scalar!(
        "SELECT id FROM app.users WHERE clerk_id = $1",
        clerk_user_id
    )
    .fetch_optional(&repo.db_pool)
    .await
    .context("failed to query database for user")?
    .ok_or_else(|| ApiError::Authentication)?;
    Ok(HttpResponse::Ok().finish())
}
