use actix_web::{
    get,
    web::{self, Json},
};

use crate::{
    errors::ApiError, features::data_providers::types::DataProviderService, postgres::PostgresRepo,
};

#[get("")]
#[tracing::instrument(skip(repo))]
pub async fn get_data_provider_services(
    repo: web::Data<PostgresRepo>,
) -> Result<Json<Vec<DataProviderService>>, ApiError> {
    let services: Vec<DataProviderService> = repo.select_all().await?;
    Ok(Json(services))
}
