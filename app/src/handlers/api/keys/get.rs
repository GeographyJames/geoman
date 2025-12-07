use actix_web::{get, web};

use domain::ApiKey;

use crate::{
    handlers::ApiError,
    helpers::get_user_context,
    postgres::PostgresRepo,
    repo::api_keys::SelectAllParams,
    types::{AuthenticatedUser, UserClient},
};

#[get("")]
#[tracing::instrument(skip(repo, user, user_client))]
pub async fn get_api_keys(
    repo: web::Data<PostgresRepo>,
    user: web::ReqData<AuthenticatedUser>,
    user_client: web::Data<UserClient>,
) -> Result<web::Json<Vec<ApiKey>>, ApiError> {
    let user_context = get_user_context(&repo, user.into_inner(), &user_client).await?;
    let params = SelectAllParams {
        user_id: user_context.id,
    };
    let keys = repo.select_all_with_params(params).await?;
    Ok(web::Json(keys.0))
}
