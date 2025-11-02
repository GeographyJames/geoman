use anyhow::Context;

use crate::app::constants::CONFIGURATION_DIRECTORY;

pub fn get_configuration_directory() -> anyhow::Result<std::path::PathBuf> {
    let base_path = std::env::current_dir().context("failed to determine current directory")?;
    Ok(base_path.join(CONFIGURATION_DIRECTORY))
}
