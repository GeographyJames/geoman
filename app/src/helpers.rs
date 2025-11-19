use actix_web::HttpRequest;
use anyhow::Context;
use domain::SupportedCrs;
use ogcapi_types::common::Crs;

use crate::{constants::CONFIGURATION_DIRECTORY, errors::ApiError, postgres::PostgresRepo};

pub fn get_configuration_directory() -> anyhow::Result<std::path::PathBuf> {
    let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = manifest_dir
        .parent()
        .context("failed to get workspace root")?;

    Ok(workspace_root.join(CONFIGURATION_DIRECTORY))
}

/// Build base URL from request
pub fn get_base_url(req: &HttpRequest) -> String {
    let connection_info = req.connection_info();
    format!("{}://{}", connection_info.scheme(), connection_info.host())
}

/// Returns a list including the default CRS and the supported CRS retrieved from the database
pub async fn supported_crs(repo: &PostgresRepo) -> Result<Vec<Crs>, ApiError> {
    let mut supported_crs = vec![Crs::default()];
    for crs in repo.select_all::<SupportedCrs>().await? {
        supported_crs.push(crs.try_into()?)
    }
    Ok(supported_crs)
}
