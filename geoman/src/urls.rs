use once_cell::sync::Lazy;
use serde::Deserialize;

pub static URLS: Lazy<Urls> = Lazy::new(|| initialise_urls().expect("failed to initialise urls"));

#[derive(Deserialize)]
pub struct Urls {
    pub health_check: String,
    pub health_check_authenticated: String,
}

fn initialise_urls() -> Result<Urls, config::ConfigError> {
    let base_path = std::env::current_dir().expect("failed to get current directory");
    let urls_directory = base_path.join("configuration");
    config::Config::builder()
        .add_source(config::File::from(urls_directory.join("urls")))
        .build()?
        .try_deserialize()
}
