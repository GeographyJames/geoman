use actix_web::{
    get,
    web::{self, Json, ReqData},
};
use domain::{User, UserId};

use crate::{errors::ApiError, postgres::PostgresRepo};

#[get("")]
#[tracing::instrument(skip(repo))]
pub async fn get_users(repo: web::Data<PostgresRepo>) -> Result<Json<Vec<User>>, ApiError> {
    let users: Vec<User> = repo.select_all().await?;
    Ok(Json(users))
}

#[get("/{user_id}")]
#[tracing::instrument(skip(repo))]
pub async fn get_user(
    repo: web::Data<PostgresRepo>,
    user_id: ReqData<UserId>,
) -> Result<Json<User>, ApiError> {
    let user: User = repo
        .select_one(user_id.into_inner())
        .await?
        .ok_or(ApiError::NotFound)?;
    Ok(Json(user))
}
