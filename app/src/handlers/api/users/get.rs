use actix_web::{
    get,
    web::{self, Json, ReqData},
};
use domain::User;

use crate::{errors::ApiError, postgres::PostgresRepo, types::AuthenticatedUser};

#[get("")]
#[tracing::instrument(skip(repo))]
pub async fn get_users(repo: web::Data<PostgresRepo>) -> Result<Json<Vec<User>>, ApiError> {
    let users: Vec<User> = repo.select_all().await?;
    Ok(Json(users))
}

#[get("/{user_id}")]
#[tracing::instrument(skip(repo, user))]
pub async fn get_user(
    repo: web::Data<PostgresRepo>,
    user: ReqData<AuthenticatedUser>,
    user_id: web::Path<String>,
) -> Result<Json<User>, ApiError> {
    if *user_id == "current" {
        tracing::info!("\n\nhere!!!!\n");
        let user: User = repo.select_one(user.id).await?.ok_or(ApiError::NotFound)?;
        return Ok(Json(user));
    }
    todo!()
}
