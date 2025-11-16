use crate::{
    URLS,
    handlers::{docs::get_api_docs, ogc_api},
};
use actix_web::web::{self, get, scope};
use clerk_rs::{
    clerk::Clerk,
    validators::{actix::ClerkMiddleware, jwks::MemoryCacheJwksProvider},
};

pub fn docs_routes(cfg: &mut web::ServiceConfig, clerk: Clerk) {
    let scp = scope(&URLS.docs.base).route(&URLS.docs.api, get().to(get_api_docs));
    cfg.service(scp.wrap(ClerkMiddleware::new(
        MemoryCacheJwksProvider::new(clerk),
        None,
        true,
    )));
}

pub fn api_routes(cfg: &mut web::ServiceConfig, clerk: Clerk) {
    let scp = scope(URLS.api.base.as_str());
    cfg.service(scp.wrap(ClerkMiddleware::new(
        MemoryCacheJwksProvider::new(clerk),
        None,
        true,
    )));
}

pub fn ogc_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        scope(URLS.ogc_api.base.as_str())
            .service(ogc_api::get_landing_page)
            .service(scope(&URLS.ogc_api.project).service(ogc_api::get_project_landing_page))
            .service(
                scope(URLS.ogc_api.conformance_declaration.as_str())
                    .service(ogc_api::get_conformance_declaration),
            )
            .service(
                scope(URLS.ogc_api.collections.as_str())
                    .service(ogc_api::get_collections)
                    .service(ogc_api::get_collection)
                    .service(ogc_api::get_features)
                    .service(ogc_api::get_feature),
            ),
    );
}
