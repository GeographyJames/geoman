use actix_web::{
    get,
    web::{self, Json},
};

use crate::{
    errors::ApiError, features::data_providers::types::DataProviderLayer, postgres::PostgresRepo,
};

#[get("")]
#[tracing::instrument(skip(repo))]
pub async fn get_data_provider_layers(
    repo: web::Data<PostgresRepo>,
) -> Result<Json<Vec<DataProviderLayer>>, ApiError> {
    let layers: Vec<DataProviderLayer> = repo.select_all().await?;
    Ok(Json(layers))
}
