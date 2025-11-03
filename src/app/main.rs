use std::net::TcpListener;

use geoman::app::{
    get_config, run,
    telemetry::{get_subscriber, init_subscriber},
};

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let config = get_config().expect("Failed to initialise app config");
    let listener = TcpListener::bind(format!(
        "{}:{}",
        config.app_settings.host, config.app_settings.port
    ))
    .expect("failed to bind to port");
    let subscriber = get_subscriber(
        "geoman".to_string(),
        "info,sqlx=error".to_string(),
        std::io::stdout,
    );
    init_subscriber(subscriber);
    tracing::info!(
        "Starting GeoMan for environment '{}' on port {}",
        config.app_settings.environment,
        config.app_settings.port
    );
    run(listener, config).expect("failed to run server").await?;
    Ok(())
}
