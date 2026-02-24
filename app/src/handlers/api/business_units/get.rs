use actix_web::{
    get,
    web::{self, Json},
};
use domain::BusinessUnit;

use crate::{errors::ApiError, postgres::PostgresRepo};

#[get("")]
#[tracing::instrument(skip(repo))]
pub async fn get_business_units(
    repo: web::Data<PostgresRepo>,
) -> Result<Json<Vec<BusinessUnit>>, ApiError> {
    let business_units: Vec<BusinessUnit> = repo.select_all().await?;
    Ok(Json(business_units))
}
