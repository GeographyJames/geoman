use actix_web::{get, web};
use clerk_rs::validators::authorizer::ClerkJwt;
use domain::ApiKey;

use crate::{handlers::ApiError, postgres::PostgresRepo, repo::api_keys::SelectAllParams};

#[get("")]
#[tracing::instrument(skip(repo))]
pub async fn get_api_keys(
    repo: web::Data<PostgresRepo>,
    jwt: web::ReqData<ClerkJwt>,
) -> Result<web::Json<Vec<ApiKey>>, ApiError> {
    let clerk_id = &jwt.sub;
    let params = SelectAllParams { clerk_id };
    let keys = repo.select_all_with_params(params).await?;
    Ok(web::Json(keys.0))
}
