use actix_web::HttpRequest;
use anyhow::Context;

use crate::{
    app::constants::{CONFIGURATION_DIRECTORY, DB_QUERY_FAIL},
    repo::{PostgresRepo, ogc::CollectionRow},
};

pub fn get_configuration_directory() -> anyhow::Result<std::path::PathBuf> {
    let base_path = std::env::current_dir().context("failed to determine current directory")?;
    Ok(base_path.join(CONFIGURATION_DIRECTORY))
}

/// Build base URL from request
pub fn get_base_url(req: &HttpRequest) -> String {
    let connection_info = req.connection_info();
    format!("{}://{}", connection_info.scheme(), connection_info.host())
}

/// Retrieve the collection ID from the given slug. Returns a 404 Not Found if no collection is found
pub async fn get_collection_row_from_slug(
    slug: &str,
    repo: &PostgresRepo,
) -> Result<CollectionRow, actix_web::Error> {
    match repo.select_by_slug(slug).await.expect(DB_QUERY_FAIL) {
        Some(row) => Ok(row),
        None => {
            Err(actix_web::error::ErrorNotFound(format!(
                "Collection id '{}' does not exist",
                slug
            )))
        }
    }
}
