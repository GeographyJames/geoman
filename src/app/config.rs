//! Application configuration and initialisation

use std::str::FromStr;

use anyhow::Context;
use dotenvy::dotenv;
use secrecy::SecretBox;
use serde::Deserialize;

use crate::app::{
    constants::ENVIRONMENT_VARIABLE_PREFIX, enums::GeoManEnvironment,
    helpers::get_configuration_directory,
};

/// Application configuration container
#[derive(Deserialize)]
pub struct AppConfig {
    pub auth_settings: ClerkAuthSettings,
    pub app_settings: AppSettings,
    pub db_settings: DatabaseSettings,
}

/// Clerk authentication settings
#[derive(Deserialize)]
pub struct ClerkAuthSettings {
    pub clerk_secret_key: SecretBox<String>,
}

/// Application settings
#[derive(Deserialize)]
pub struct AppSettings {
    pub environment: GeoManEnvironment,
    pub host: String,
    pub port: u16,
}

/// PostgreSQL database settings
#[derive(Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: SecretBox<String>,
    pub port: u16,
    pub host: String,
    pub database_name: String,
    pub require_ssl: bool,
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
