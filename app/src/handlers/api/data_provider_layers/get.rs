use actix_web::{
    get,
    web::{self, Json},
};
use domain::DataProviderLayer;

use crate::{errors::ApiError, postgres::PostgresRepo};

#[get("")]
#[tracing::instrument(skip(repo))]
pub async fn get_data_provider_layers(
    repo: web::Data<PostgresRepo>,
) -> Result<Json<Vec<DataProviderLayer>>, ApiError> {
    let layers: Vec<DataProviderLayer> = repo.select_all().await?;
    Ok(Json(layers))
}
