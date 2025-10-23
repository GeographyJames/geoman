use std::str::FromStr;

use anyhow::Context;
use dotenvy::dotenv;
use secrecy::SecretBox;
use serde::Deserialize;
use strum::{self, Display, EnumString};

use crate::{constants::GEOMAN_ENVIRONMENT_KEY, helpers::get_configuration_directory};

#[derive(Deserialize)]
pub struct AppConfig {
    pub auth: ClerkAuth,
    pub app_settings: ApplicationSettings,
}

#[derive(Deserialize)]
pub struct ClerkAuth {
    pub clerk_secret_key: SecretBox<String>,
}

#[derive(Deserialize)]
pub struct ApplicationSettings {
    pub environment: Environment,
    pub host: String,
    pub port: u16,
}

#[derive(Deserialize, EnumString, Display, Clone)]
#[strum(ascii_case_insensitive, serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum Environment {
    Local,
    Demo,
    Staging,
    Production,
}

pub fn get_config() -> Result<AppConfig, anyhow::Error> {
    dotenv().ok();
    let environment = Environment::from_str(
        &std::env::var(GEOMAN_ENVIRONMENT_KEY)
            .map_err(|e| anyhow::anyhow!("no {GEOMAN_ENVIRONMENT_KEY} set: {}", e))?,
    )
    .context(format!("failed to parse {GEOMAN_ENVIRONMENT_KEY}"))?;
    let configuration_directory = get_configuration_directory();
    let environment_filename = format!("{}.yaml", environment);
    let config_builder = config::Config::builder();
    let config = config_builder
        .set_default("app_settings.environment", environment.to_string())
        .context("failed to add environment to config builder")?
        .add_source(
            config::Environment::with_prefix("GEOMAN")
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
