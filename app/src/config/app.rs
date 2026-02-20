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
#[derive(Deserialize, Clone)]
pub struct AppConfig {
    pub auth_settings: ClerkAuthSettings,
    pub app_settings: AppSettings,
    pub db_settings: DatabaseSettings,
}

#[derive(Deserialize, Clone)]
pub struct Environment {
    pub run: GeoManEnvironment,
    pub config: GeoManEnvironment,
}

/// Application settings
#[derive(Deserialize, Clone)]
pub struct AppSettings {
    pub environment: Environment,

    pub host: String,
    pub port: u16,
}

/// Creates application configuration from YAML configuratiton files for specific runtime environment.
pub fn get_config() -> Result<AppConfig, anyhow::Error> {
    dotenv().ok();
    let environment = get_environment().context("failed to determine app environment")?;
    let configuration_directory =
        get_configuration_directory().context("failed to determint configuration directory")?;
    let environment_filename = format!("{}.yaml", environment.config);
    let config_builder = config::Config::builder();
    let config = config_builder
        .set_default(
            "app_settings.environment.config",
            environment.config.to_string(),
        )
        .context("failed to add config environment to config builder")?
        .set_default("app_settings.environment.run", environment.run.to_string())
        .context("failed to add run environment to config builder")?
        .add_source(config::File::from(
            configuration_directory.join(environment_filename),
        ))
        .add_source(
            config::Environment::with_prefix(ENVIRONMENT_VARIABLE_PREFIX)
                .prefix_separator("_")
                .separator("__"),
        )
        .build()
        .context("failed to build config")?;

    let app_config = config
        .try_deserialize::<AppConfig>()
        .context("failed to deserialise app config")?;
    Ok(app_config)
}

fn get_environment() -> anyhow::Result<Environment> {
    let run_env_key = format!("{ENVIRONMENT_VARIABLE_PREFIX}_RUN_ENVIRONMENT");
    let config_env_key = format!("{ENVIRONMENT_VARIABLE_PREFIX}_CONFIG_ENVIRONMENT");
    Ok(Environment {
        run: env_from_key(run_env_key)?,
        config: env_from_key(config_env_key)?,
    })
}

fn env_from_key(key: String) -> anyhow::Result<GeoManEnvironment> {
    GeoManEnvironment::from_str(
        &std::env::var(&key)
            .map_err(|e| anyhow::anyhow!("no '{key}' environment variable set: {}", e))?,
    )
    .context(format!("failed to parse {key} environment variable"))
}
