use actix_web::{get, web};
use domain::{ApiKey, UserId};

use crate::{handlers::ApiError, postgres::PostgresRepo, repo::api_keys::SelectAllParams};

#[get("")]
#[tracing::instrument(skip(repo, user_id))]
pub async fn get_api_keys(
    repo: web::Data<PostgresRepo>,
    user_id: web::ReqData<UserId>,
) -> Result<web::Json<Vec<ApiKey>>, ApiError> {
    let params = SelectAllParams {
        user_id: user_id.into_inner(),
    };
    let keys = repo.select_all_with_params(params).await?;
    Ok(web::Json(keys.0))
}
