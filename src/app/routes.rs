use actix_web::{
    HttpResponse,
    web::{self, ServiceConfig},
};

use crate::app::{URLS, handlers::api::projects::get_projects};

pub fn unprotected_routes(cfg: &mut ServiceConfig) {
    cfg.route(&URLS.health_check, web::get().to(HttpResponse::Ok));
}

pub fn protected_routes(cfg: &mut ServiceConfig) {
    cfg.route(
        &URLS.health_check_authenticated,
        web::get().to(HttpResponse::Ok),
    )
    .service(web::scope(&URLS.api.base).configure(api_routes));
}

pub fn api_routes(cfg: &mut ServiceConfig) {
    cfg.route(&URLS.api.projects, web::get().to(get_projects));
}
