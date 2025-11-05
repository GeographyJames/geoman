use crate::app::{
    URLS,
    handlers::{api::projects::get_projects, docs::get_api_docs, ogc_api},
};
use clerk_rs::{
    clerk::Clerk,
    validators::{actix::ClerkMiddleware, jwks::MemoryCacheJwksProvider},
};

pub fn docs_routes(cfg: &mut actix_web::web::ServiceConfig, clerk: Clerk) {
    let scp = actix_web::web::scope(&URLS.docs.base)
        .route(&URLS.docs.api, actix_web::web::get().to(get_api_docs));
    cfg.service(scp.wrap(ClerkMiddleware::new(
        MemoryCacheJwksProvider::new(clerk),
        None,
        true,
    )));
}

pub fn api_routes(cfg: &mut utoipa_actix_web::service_config::ServiceConfig, clerk: Clerk) {
    let scp = utoipa_actix_web::scope::scope(URLS.api.base.as_str()).configure(project_routes);
    cfg.service(scp.wrap(ClerkMiddleware::new(
        MemoryCacheJwksProvider::new(clerk),
        None,
        true,
    )));
}

pub fn project_routes(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.service(utoipa_actix_web::scope::scope(URLS.api.projects.as_str()).service(get_projects));
}

pub fn ogc_routes(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.service(
        utoipa_actix_web::scope::scope(URLS.ogc_api.base.as_str())
            .service(ogc_api::get_landing_page)
            .service(
                utoipa_actix_web::scope::scope(URLS.ogc_api.conformance_declaration.as_str())
                    .service(ogc_api::get_conformance_declaration),
            ),
    );
}
