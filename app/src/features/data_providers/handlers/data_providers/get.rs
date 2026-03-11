use actix_web::{
    get,
    web::{self, Json},
};

use crate::{
    errors::ApiError, features::data_providers::types::DataProvider, postgres::PostgresRepo,
};

#[get("")]
#[tracing::instrument(skip(repo))]
pub async fn get_data_providers(
    repo: web::Data<PostgresRepo>,
) -> Result<Json<Vec<DataProvider>>, ApiError> {
    let providers: Vec<DataProvider> = repo.select_all().await?;
    Ok(Json(providers))
}
