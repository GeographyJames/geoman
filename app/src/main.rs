use app::{
    Application, get_config,
    telemetry::{get_subscriber, init_subscriber},
};

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let config = get_config().expect("Failed to initialise app config");
    let subscriber = get_subscriber(
        "GeoMan".to_string(),
        "info,sqlx=error".to_string(),
        std::io::stdout,
    );
    init_subscriber(subscriber);
    tracing::info!(
        "\nStarting GeoMan 🌍\nConfig environment: {}\nRun environment: {}\nPort: {}\n",
        config.app_settings.environment.config,
        config.app_settings.environment.run,
        config.app_settings.port
    );
    let app = Application::build(config)
        .await
        .expect("Failed to build application");

    app.run_untill_stopped()
        .await
        .expect("Failed to run application");

    Ok(())
}
