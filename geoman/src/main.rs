use std::net::TcpListener;

use geoman::app::{config::get_config, startup::run};

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let config = get_config().expect("Failed to initialise app config");
    let listener = TcpListener::bind(format!(
        "{}:{}",
        config.app_settings.host, config.app_settings.port
    ))
    .expect("failed to bind to port");
    println!(
        "Starting GeoMan for {} environment on port {}",
        config.app_settings.environment, config.app_settings.port
    );
    run(listener, &config)?.await?;
    Ok(())
}
