use actix_web::{HttpResponse, http::StatusCode, patch, web};

use domain::KeyId;

use crate::{
    handlers::ApiError,
    helpers::get_user_context,
    postgres::PostgresRepo,
    types::{AuthenticatedUser, UserClient},
};

#[patch("/{id}/renew")]
#[tracing::instrument(skip(repo, id, user, user_client))]
pub async fn renew_api_key(
    repo: web::Data<PostgresRepo>,
    id: web::Path<KeyId>,
    user: web::ReqData<AuthenticatedUser>,
    user_client: web::Data<UserClient>,
) -> Result<HttpResponse, ApiError> {
    let user_context = get_user_context(&repo, user.into_inner(), &user_client).await?;
    repo.renew_api_key(id.into_inner(), user_context.id).await?;
    Ok(HttpResponse::new(StatusCode::NO_CONTENT))
}
