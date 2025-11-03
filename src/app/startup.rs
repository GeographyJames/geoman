use crate::app::{
    URLS,
    config::AppConfig,
    routes::{api_routes, docs_routes},
};
use actix_web::{App, HttpResponse, HttpServer, dev::Server, web};
use anyhow::Context;
use clerk_rs::{ClerkConfiguration, clerk::Clerk};
use secrecy::ExposeSecret;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;
use utoipa_actix_web::AppExt;

pub fn run(listener: TcpListener, config: AppConfig) -> anyhow::Result<Server> {
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

    let server = HttpServer::new(move || {
        let (app, api_docs) = App::new()
            .wrap(TracingLogger::default())
            .route(&URLS.health_check, web::get().to(HttpResponse::Ok))
            .into_utoipa_app()
            .configure(|cfg| api_routes(cfg, clerk.clone()))
            .split_for_parts();
        let app = app
            .app_data(web::Data::new(api_docs))
            .configure(|cfg| docs_routes(cfg, clerk.clone()));
        app
    })
    .listen(listener)
    .context("failed to bind to listener")?
    .run();
    Ok(server)
}
