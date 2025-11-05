use geoman::app::{
    Application, get_config,
    telemetry::{get_subscriber, init_subscriber},
};

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let config = get_config().expect("Failed to initialise app config");
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
    let app = Application::build(config)
        .await
        .expect("failed to build application");

    app.run_untill_stopped()
        .await
        .expect("failed to run application");

    Ok(())
}
