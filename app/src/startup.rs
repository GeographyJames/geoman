use crate::{
    AppState, URLS,
    config::AppConfig,
    handlers::webhooks::clerk_webhook,
    postgres::PostgresRepo,
    routes::{api_routes, ogc_routes},
};
use actix_web::{App, HttpResponse, HttpServer, dev::Server, web};
use anyhow::Context;
use clerk_rs::{
    ClerkConfiguration,
    clerk::Clerk,
    validators::{authorizer::ClerkAuthorizer, jwks::MemoryCacheJwksProvider},
};
use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;
use utoipa_swagger_ui::SwaggerUi;

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

    let app_state = AppState::new();
    let openapi = app_state.openapi.clone();
    let app_state = web::Data::new(app_state);
    let repo = web::Data::new(PostgresRepo::new(db_pool));
    let clerk_authoriser = web::Data::new(ClerkAuthorizer::new(
        MemoryCacheJwksProvider::new(clerk.clone()),
        true,
    ));

    let server = HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .app_data(repo.clone())
            .app_data(clerk_authoriser.clone())
            .wrap(TracingLogger::default())
            .route(
                &format!("{}{}", URLS.webhooks.base, URLS.webhooks.clerk),
                web::post().to(clerk_webhook),
            )
            .route(&URLS.health_check, web::get().to(HttpResponse::Ok))
            .configure(|cfg| {
                api_routes(
                    cfg,
                    clerk.clone(),
                    config.app_settings.environment.run.clone(),
                )
            })
            .configure(|cfg| ogc_routes(cfg, config.app_settings.environment.run.clone()))
            // .configure(|cfg| {
            //     docs_routes(cfg, clerk.clone(), config.app_settings.environment.clone())
            // })
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/ogcapi/openapi.json", openapi.clone()),
            )
            // .service(
            //     actix_files::Files::new("/book", "geoman-book/book")
            //         .index_file("index.html")
            //         .redirect_to_slash_directory(),
            // )
            .service(
                actix_web_lab::web::spa()
                    .index_file("./react-frontend/dist/index.html")
                    .static_resources_location("./react-frontend/dist")
                    .static_resources_mount("/")
                    .finish(),
            )
    })
    .listen(listener)
    .context("failed to bind to listener")?
    .run();
    Ok(server)
}
