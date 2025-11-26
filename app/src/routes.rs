use crate::{URLS, handlers::ogc_api};
use actix_web::{
    middleware,
    web::{self, scope},
};
use clerk_rs::{
    clerk::Clerk,
    validators::{actix::ClerkMiddleware, jwks::MemoryCacheJwksProvider},
};

// pub fn docs_routes(cfg: &mut web::ServiceConfig, clerk: Clerk, environment: GeoManEnvironment) {
//     let scp = scope(&URLS.docs.base);
//     match environment {
//         GeoManEnvironment::Development => {
//             cfg.service(scp);
//         }
//         _ => {
//             cfg.service(scp.wrap(ClerkMiddleware::new(
//                 MemoryCacheJwksProvider::new(clerk),
//                 None,
//                 true,
//             )));
//         }
//     }
// }

pub fn api_routes(cfg: &mut web::ServiceConfig, clerk: Clerk) {
    let scp = scope(&URLS.api.base);
    cfg.service(scp.wrap(ClerkMiddleware::new(
        MemoryCacheJwksProvider::new(clerk),
        None,
        true,
    )));
}

pub fn ogc_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        scope(&URLS.ogc_api.base)
            .wrap(middleware::NormalizePath::trim()) // required to pass OGC Features API test suit.
            .service(scope(&URLS.ogc_api.project).configure(project_ogc_routes))
            .service(ogc_api::get_landing_page)
            .service(scope(&URLS.ogc_api.openapi).service(ogc_api::get_openapi))
            .service(
                scope(&URLS.ogc_api.conformance_declaration)
                    .service(ogc_api::get_conformance_declaration),
            )
            .service(
                scope(&URLS.ogc_api.collections)
                    .service(ogc_api::get_collections)
                    .service(ogc_api::get_collection)
                    .service(ogc_api::get_features)
                    .service(ogc_api::get_feature),
            ),
    );
}

pub fn project_ogc_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        scope("/{project}")
            .wrap(middleware::NormalizePath::trim()) // required to pass OGC Features API test suit.
            .service(ogc_api::get_project_landing_page)
            .service(scope(&URLS.ogc_api.openapi).service(ogc_api::get_openapi))
            .service(
                scope(&URLS.ogc_api.conformance_declaration)
                    .service(ogc_api::get_project_conformance_declaration),
            )
            .service(
                scope(&URLS.ogc_api.collections)
                    .service(ogc_api::get_project_collections)
                    .service(ogc_api::get_project_collection)
                    .service(ogc_api::get_project_features)
                    .service(ogc_api::get_project_feature),
            ),
    );
}
