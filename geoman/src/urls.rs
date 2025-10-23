use once_cell::sync::Lazy;
use serde::Deserialize;

use crate::helpers::get_configuration_directory;

pub static URLS: Lazy<Urls> = Lazy::new(|| initialise_urls().expect("failed to initialise urls"));

#[derive(Deserialize)]
pub struct Urls {
    pub health_check: String,
    pub health_check_authenticated: String,
}

fn initialise_urls() -> Result<Urls, config::ConfigError> {
    let configuration_directory = get_configuration_directory();
    config::Config::builder()
        .add_source(config::File::from(configuration_directory.join("urls")))
        .build()?
        .try_deserialize()
}
