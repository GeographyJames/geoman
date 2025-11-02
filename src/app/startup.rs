use actix_web::{App, HttpServer, dev::Server, web};
use anyhow::Context;
use clerk_rs::{
    ClerkConfiguration,
    clerk::Clerk,
    validators::{actix::ClerkMiddleware, jwks::MemoryCacheJwksProvider},
};
use secrecy::ExposeSecret;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

use crate::app::{
    config::AppConfig,
    routes::{protected_routes, unprotected_routes},
};

pub fn run(listener: TcpListener, config: &AppConfig) -> anyhow::Result<Server> {
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
        App::new()
            .wrap(TracingLogger::default())
            .configure(unprotected_routes)
            .service(
                web::scope("")
                    .wrap(ClerkMiddleware::new(
                        MemoryCacheJwksProvider::new(clerk.clone()),
                        None,
                        true,
                    ))
                    .configure(protected_routes),
            )
    })
    .listen(listener)
    .context("failed to bind to listener")?
    .run();
    Ok(server)
}
