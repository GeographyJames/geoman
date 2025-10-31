use actix_web::{App, HttpResponse, HttpServer, dev::Server, web};
use anyhow::Context;
use clerk_rs::{
    ClerkConfiguration,
    clerk::Clerk,
    validators::{actix::ClerkMiddleware, jwks::MemoryCacheJwksProvider},
};
use secrecy::ExposeSecret;
use std::net::TcpListener;

use crate::app::{config::AppConfig, urls::URLS};

pub fn run(listener: TcpListener, config: &AppConfig) -> anyhow::Result<Server> {
    let clerk_config = ClerkConfiguration::new(
        None,
        None,
        Some(config.auth.clerk_secret_key.expose_secret().clone()),
        None,
    );
    let clerk = Clerk::new(clerk_config);
    let server = HttpServer::new(move || {
        App::new()
            .route(&URLS.health_check, web::get().to(HttpResponse::Ok))
            .service(
                web::resource(&URLS.health_check_authenticated)
                    .wrap(ClerkMiddleware::new(
                        MemoryCacheJwksProvider::new(clerk.clone()),
                        None,
                        true,
                    ))
                    .route(web::get().to(HttpResponse::Ok)),
            )
    })
    .listen(listener)
    .context("failed to bind to listener")?
    .run();
    Ok(server)
}
