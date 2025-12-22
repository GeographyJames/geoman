use actix_web::{
    get,
    web::{self, Json},
};
use domain::Technology;
use serde::{Deserialize, Serialize};

use crate::{errors::ApiError, postgres::PostgresRepo};

#[derive(Serialize, Deserialize)]
pub struct AppSettings {
    pub technologies: Vec<Technology>,
}

#[tracing::instrument(skip(repo))]
pub async fn get_app_settings(
    repo: web::Data<PostgresRepo>,
) -> Result<Json<AppSettings>, ApiError> {
    let technologies = repo.select_all().await?;
    Ok(Json(AppSettings { technologies }))
}
