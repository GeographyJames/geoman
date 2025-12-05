use actix_web::{get, web};
use domain::ApiKey;

use crate::{
    handlers::ApiError, postgres::PostgresRepo, repo::api_keys::SelectAllParams,
    types::AuthenticatedUser,
};

#[get("")]
#[tracing::instrument(skip(repo, user))]
pub async fn get_api_keys(
    repo: web::Data<PostgresRepo>,
    user: web::ReqData<AuthenticatedUser>,
) -> Result<web::Json<Vec<ApiKey>>, ApiError> {
    let params = SelectAllParams { user_id: user.id };
    let keys = repo.select_all_with_params(params).await?;
    Ok(web::Json(keys.0))
}
