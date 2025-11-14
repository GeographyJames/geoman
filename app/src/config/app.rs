use crate::{
    config::{clerk::ClerkAuthSettings, db::DatabaseSettings},
    constants::ENVIRONMENT_VARIABLE_PREFIX,
    enums::GeoManEnvironment,
    helpers::get_configuration_directory,
};
use anyhow::Context;
use dotenvy::dotenv;
use serde::Deserialize;
use std::str::FromStr;

/// Application configuration container
#[derive(Deserialize)]
pub struct AppConfig {
    pub auth_settings: ClerkAuthSettings,
    pub app_settings: AppSettings,
    pub db_settings: DatabaseSettings,
}

/// Application settings
#[derive(Deserialize, Clone)]
pub struct AppSettings {
    pub environment: GeoManEnvironment,
    pub host: String,
    pub port: u16,
}

/// Creates application configuration from YAML configuratiton files for specific runtime environment.
pub fn get_config() -> Result<AppConfig, anyhow::Error> {
    dotenv().ok();
    let environment = get_environment().context("failed to determine app environment")?;
    let configuration_directory =
        get_configuration_directory().context("failed to determint configuration directory")?;
    let environment_filename = format!("{}.yaml", environment);
    let config_builder = config::Config::builder();
    let config = config_builder
        .set_default("app_settings.environment", environment.to_string())
        .context("failed to add environment to config builder")?
        .add_source(
            config::Environment::with_prefix(ENVIRONMENT_VARIABLE_PREFIX)
                .prefix_separator("_")
                .separator("__"),
        )
        .add_source(config::File::from(
            configuration_directory.join(environment_filename),
        ))
        .build()
        .context("failed to build config")?;

    let app_config = config
        .try_deserialize::<AppConfig>()
        .context("failed to deserialise app config")?;
    Ok(app_config)
}

fn get_environment() -> anyhow::Result<GeoManEnvironment> {
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
