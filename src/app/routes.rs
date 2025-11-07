use crate::app::{
    URLS,
    handlers::{docs::get_api_docs, ogc_api},
};
use actix_web::web::{get, scope as actix_scope};
use clerk_rs::{
    clerk::Clerk,
    validators::{actix::ClerkMiddleware, jwks::MemoryCacheJwksProvider},
};
use utoipa_actix_web::scope as utoipa_scope;

pub fn docs_routes(cfg: &mut actix_web::web::ServiceConfig, clerk: Clerk) {
    let scp = actix_scope(&URLS.docs.base).route(&URLS.docs.api, get().to(get_api_docs));
    cfg.service(scp.wrap(ClerkMiddleware::new(
        MemoryCacheJwksProvider::new(clerk),
        None,
        true,
    )));
}

pub fn api_routes(cfg: &mut utoipa_actix_web::service_config::ServiceConfig, clerk: Clerk) {
    let scp = utoipa_actix_web::scope::scope(URLS.api.base.as_str());
    cfg.service(scp.wrap(ClerkMiddleware::new(
        MemoryCacheJwksProvider::new(clerk),
        None,
        true,
    )));
}

pub fn ogc_routes(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.service(
        utoipa_scope(URLS.ogc_api.base.as_str())
            .service(ogc_api::get_landing_page)
            .service(
                utoipa_scope(URLS.ogc_api.conformance_declaration.as_str())
                    .service(ogc_api::get_conformance_declaration),
            )
            .service(
                utoipa_scope(URLS.ogc_api.collections.as_str())
                    .service(ogc_api::get_collections)
                    .service(ogc_api::get_collection)
                    .service(ogc_api::get_features)
                    .service(ogc_api::get_feature),
            ),
    );
}
