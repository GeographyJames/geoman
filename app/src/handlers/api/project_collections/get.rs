use actix_web::{
    get,
    web::{self, Json},
};
use domain::CollectionListItem;

use crate::{errors::ApiError, postgres::PostgresRepo};

#[get("")]
#[tracing::instrument(skip(repo))]
pub async fn get_collections(
    repo: web::Data<PostgresRepo>,
) -> Result<Json<Vec<CollectionListItem>>, ApiError> {
    let collections: Vec<CollectionListItem> = repo.select_all().await?;
    Ok(Json(collections))
}
