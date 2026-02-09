use actix_web::HttpRequest;
use anyhow::Context;
use domain::KeyHash;
use secrecy::{ExposeSecret, SecretBox};
use sha2::{Digest, Sha256};

use crate::constants::CONFIGURATION_DIRECTORY;

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

/// Hash an API key using SHA256 for database storage
pub fn hash_api_key(api_key: &SecretBox<String>) -> KeyHash {
    let mut hasher = Sha256::new();
    hasher.update(api_key.expose_secret().as_bytes());
    KeyHash(hex::encode(hasher.finalize()))
}
