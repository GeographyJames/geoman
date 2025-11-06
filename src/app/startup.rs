use crate::app::{
    AppState, URLS,
    config::AppConfig,
    routes::{api_routes, docs_routes, ogc_routes},
};
use actix_web::{App, HttpResponse, HttpServer, dev::Server, web};
use anyhow::Context;
use clerk_rs::{ClerkConfiguration, clerk::Clerk};
use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;
use utoipa_actix_web::AppExt;

pub struct Application {
    pub server: Server,
    pub port: u16,
}

impl Application {
    pub async fn build(config: AppConfig) -> anyhow::Result<Self> {
        let db_pool = config.db_settings.get_connection_pool();
        let listener = TcpListener::bind(format!(
            "{}:{}",
            config.app_settings.host, config.app_settings.port
        ))
        .context("failed to bind to port")?;
        let port = listener.local_addr().unwrap().port();
        let server = run(listener, config, db_pool)
            .await
            .context("failed to run server")?;

        Ok(Self { server, port })
    }
    pub async fn run_untill_stopped(self) -> std::io::Result<()> {
        self.server.await
    }
}

pub async fn run(
    listener: TcpListener,
    config: AppConfig,
    db_pool: PgPool,
) -> anyhow::Result<Server> {
    let clerk_config = ClerkConfiguration::new(
        None,
        None,
        Some(
            config
                .auth_settings
                .clerk_secret_key
                .expose_secret()
                .clone(),
        ),
        None,
    );
    let clerk = Clerk::new(clerk_config);
    let app_state = web::Data::new(AppState::new(db_pool));

    let server = HttpServer::new(move || {
        let (app, api_docs) = App::new()
            .app_data(app_state.clone())
            .wrap(TracingLogger::default())
            .route(&URLS.health_check, web::get().to(HttpResponse::Ok))
            .into_utoipa_app()
            .configure(|cfg| api_routes(cfg, clerk.clone()))
            .configure(ogc_routes)
            .split_for_parts();

        app.app_data(web::Data::new(api_docs))
            .configure(|cfg| docs_routes(cfg, clerk.clone()))
    })
    .listen(listener)
    .context("failed to bind to listener")?
    .run();
    Ok(server)
}
