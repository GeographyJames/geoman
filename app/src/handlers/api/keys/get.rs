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
    let auth_id = match user.into_inner() {
        AuthenticatedUser::AuthenticationId(id) => id,
        AuthenticatedUser::User(_) => {
            return Err(ApiError::Unexpected(anyhow::anyhow!(
                "Expected AuthenticationId, got User context"
            )));
        }
    };
    let params = SelectAllParams { auth_id: &auth_id };
    let keys = repo.select_all_with_params(params).await?;
    Ok(web::Json(keys.0))
}
