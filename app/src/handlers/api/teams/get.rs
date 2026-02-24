use actix_web::{
    get,
    web::{self, Json},
};
use domain::Team;

use crate::{errors::ApiError, postgres::PostgresRepo};

#[get("")]
#[tracing::instrument(skip(repo))]
pub async fn get_teams(repo: web::Data<PostgresRepo>) -> Result<Json<Vec<Team>>, ApiError> {
    let teams: Vec<Team> = repo.select_all().await?;
    Ok(Json(teams))
}
