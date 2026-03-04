use actix_web::{
    get,
    web::{self, Json},
};
use domain::DataProvider;

use crate::{errors::ApiError, postgres::PostgresRepo};

#[get("")]
#[tracing::instrument(skip(repo))]
pub async fn get_data_providers(
    repo: web::Data<PostgresRepo>,
) -> Result<Json<Vec<DataProvider>>, ApiError> {
    let providers: Vec<DataProvider> = repo.select_all().await?;
    Ok(Json(providers))
}
