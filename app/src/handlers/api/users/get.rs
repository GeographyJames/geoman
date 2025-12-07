use actix_web::{
    get,
    web::{self, Json, ReqData},
};
use domain::User;

use crate::{
    errors::ApiError,
    helpers::get_user_context,
    postgres::PostgresRepo,
    types::{AuthenticatedUser, UserClient},
};

#[get("")]
#[tracing::instrument(skip(repo))]
pub async fn get_users(repo: web::Data<PostgresRepo>) -> Result<Json<Vec<User>>, ApiError> {
    let users: Vec<User> = repo.select_all().await?;
    Ok(Json(users))
}

#[get("/{user_id}")]
#[tracing::instrument(skip(repo, user, user_client))]
pub async fn get_user(
    repo: web::Data<PostgresRepo>,
    user: ReqData<AuthenticatedUser>,
    user_id: web::Path<String>,
    user_client: web::Data<UserClient>,
) -> Result<Json<User>, ApiError> {
    let user_context = get_user_context(&repo, user.into_inner(), &user_client).await?;
    if *user_id == "current" {
        let user: User = repo
            .select_one(user_context.id)
            .await?
            .ok_or(ApiError::NotFound)?;
        return Ok(Json(user));
    }
    todo!()
}
