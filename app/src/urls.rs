use crate::helpers::get_configuration_directory;
use anyhow::Context;
use serde::Deserialize;
use std::sync::LazyLock;

pub static URLS: LazyLock<Urls> =
    LazyLock::new(|| initialise_urls().expect("failed to initialise urls"));

#[derive(Deserialize)]
pub struct Urls {
    pub health_check: String,
    pub api: Api,
    pub docs: Docs,
    pub ogc_api: Ogcapi,
}

#[derive(Deserialize)]
pub struct Api {
    pub base: String,
    pub keys: String,
}

#[derive(Deserialize)]
pub struct Docs {
    pub base: String,
}

#[derive(Deserialize)]
pub struct Ogcapi {
    pub base: String,
    pub conformance_declaration: String,
    pub collections: String,
    pub project: String,
    pub openapi: String,
}

fn initialise_urls() -> Result<Urls, anyhow::Error> {
    let configuration_directory =
        get_configuration_directory().context("failed to determine configuration directory")?;
    config::Config::builder()
        .add_source(config::File::from(configuration_directory.join("urls")))
        .build()
        .context("failed to build URLs config")?
        .try_deserialize()
        .context("failed to deserialise URLs")
}
