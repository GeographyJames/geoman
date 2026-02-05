use crate::{
    URLS,
    enums::GeoManEnvironment,
    handlers::ogc_api,
    middleware::{dual_auth_middleware, mock_auth_middlewear},
};
use actix_web::{
    middleware,
    web::{self, scope},
};

pub fn ogc_routes(cfg: &mut web::ServiceConfig, run_environment: GeoManEnvironment) {
    let scp = scope(&URLS.ogc_api.base)
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
                .service(ogc_api::get_feature)
                .service(ogc_api::get_collection_queryables),
        );

    match run_environment {
        GeoManEnvironment::Production => {
            cfg.service(scp.wrap(middleware::from_fn(dual_auth_middleware)));
        }
        GeoManEnvironment::Development => {
            cfg.service(scp.wrap(middleware::from_fn(mock_auth_middlewear)));
        }
        _ => {
            cfg.service(scp);
        }
    }
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
