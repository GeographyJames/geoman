use crate::{
    URLS,
    enums::GeoManEnvironment,
    handlers::api::{
        app_settings::get_app_settings,
        features::patch::patch_project_feature,
        keys::{generate_api_key, get_api_keys, renew_api_key, revoke_api_key},
        project_collections::{get_collections, post_project_collection},
        projects::{patch_project, post_project},
        users::{get_user, get_users},
    },
    middleware::{auth_middleware, mock_auth_middlewear},
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
        .configure(project_features_routes)
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
    cfg.service(
        scope(&URLS.api.collections)
            .service(get_collections)
            .service(post_project_collection),
    );
}

pub fn project_features_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(scope(&URLS.api.project_features).service(patch_project_feature));
}
