use std::str::FromStr;

use anyhow::Context;

use crate::app::{
    constants::{CONFIGURATION_DIRECTORY, ENVIRONMENT_VARIABLE_PREFIX},
    enums::GeoManEnvironment,
};

pub fn get_configuration_directory() -> anyhow::Result<std::path::PathBuf> {
    let base_path = std::env::current_dir().context("failed to determine current directory")?;
    Ok(base_path.join(CONFIGURATION_DIRECTORY))
}

pub fn get_environment() -> anyhow::Result<GeoManEnvironment> {
    let geoman_env_key = format!("{ENVIRONMENT_VARIABLE_PREFIX}_ENVIRONMENT");
    GeoManEnvironment::from_str(
        &std::env::var(&geoman_env_key).map_err(|e| {
            anyhow::anyhow!("no '{geoman_env_key}' environment variable set: {}", e)
        })?,
    )
    .context(format!(
        "failed to parse {geoman_env_key} environment variable"
    ))
}
