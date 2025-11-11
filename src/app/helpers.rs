use actix_web::HttpRequest;
use anyhow::Context;

use crate::constants::CONFIGURATION_DIRECTORY;

pub fn get_configuration_directory() -> anyhow::Result<std::path::PathBuf> {
    let base_path = std::env::current_dir().context("failed to determine current directory")?;
    Ok(base_path.join(CONFIGURATION_DIRECTORY))
}

/// Build base URL from request
pub fn get_base_url(req: &HttpRequest) -> String {
    let connection_info = req.connection_info();
    format!("{}://{}", connection_info.scheme(), connection_info.host())
}
