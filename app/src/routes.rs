use crate::{
    URLS,
    enums::GeoManEnvironment,
    handlers::{
        api::{
            app_settings::get_app_settings,
            keys::{generate_api_key, get_api_keys, renew_api_key, revoke_api_key},
            project_collections::post_project_collection,
            projects::{patch_project, post_project},
            users::{get_user, get_users},
        },
        ogc_api,
    },
    middleware::{auth_middleware, dual_auth_middleware, mock_auth_middlewear},
};
use actix_web::{
    middleware,
    web::{self, scope},
};
use clerk_rs::clerk::Clerk;

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

pub fn api_routes(cfg: &mut web::ServiceConfig, _clerk: Clerk, run_environment: GeoManEnvironment) {
    let scp = scope(&URLS.api.base)
        .configure(api_key_routes)
        .configure(project_routes)
        .configure(user_routes)
        .configure(project_collection_routes)
        .route(&URLS.api.app_settings, web::get().to(get_app_settings));
    match run_environment {
        GeoManEnvironment::Development => {
            cfg.service(scp.wrap(middleware::from_fn(mock_auth_middlewear)));
        }
        _ => {
            cfg.service(scp.wrap(middleware::from_fn(auth_middleware)));
        }
    };
}

pub fn api_key_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        scope(&URLS.api.keys)
            .service(generate_api_key)
            .service(get_api_keys)
            .service(revoke_api_key)
            .service(renew_api_key),
    );
}

pub fn project_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        scope(&URLS.api.projects)
            .service(post_project)
            .service(patch_project),
    );
}

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(scope(&URLS.api.users).service(get_users).service(get_user));
}

pub fn project_collection_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(scope(&URLS.api.collections).service(post_project_collection));
}

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
                    .service(ogc_api::get_project_feature)
                    .service(ogc_api::patch_project_feature),
            ),
    );
}
