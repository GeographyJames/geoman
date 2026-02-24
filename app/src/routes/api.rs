use crate::{
    URLS,
    enums::GeoManEnvironment,
    handlers::api::{
        business_units::{get_business_units, post_business_unit},
        epsg::{post_epsg, post_epsg_from_shz},
        features::{
            get::get_project_feature_shapefile, patch::patch_project_feature,
            post::post_project_feature_shapefile,
        },
        keys::{generate_api_key, get_api_keys, renew_api_key, revoke_api_key},
        project_collections::{get_collections, patch_collection, post_project_collection},
        projects::{patch_project, post_project},
        teams::{delete_team, get_teams, patch_team, post_team},
        users::{get_user, get_users, patch_user},
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
        .configure(epsg_routes)
        .configure(teams_routes)
        .configure(business_units_routes);

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
    cfg.service(
        scope(&URLS.api.users)
            .service(get_users)
            .service(get_user)
            .service(patch_user),
    );
}

pub fn project_collection_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        scope(&URLS.api.collections)
            .service(get_collections)
            .service(post_project_collection)
            .service(patch_collection),
    );
}

pub fn project_features_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        scope(&URLS.api.project_features)
            .service(patch_project_feature)
            .service(post_project_feature_shapefile)
            .service(get_project_feature_shapefile),
    );
}

pub fn epsg_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        scope(&URLS.api.epsg)
            .service(post_epsg)
            .service(post_epsg_from_shz),
    );
}

pub fn teams_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        scope(&URLS.api.teams)
            .service(get_teams)
            .service(post_team)
            .service(patch_team)
            .service(delete_team),
    );
}

pub fn business_units_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        scope(&URLS.api.business_units)
            .service(get_business_units)
            .service(post_business_unit),
    );
}
