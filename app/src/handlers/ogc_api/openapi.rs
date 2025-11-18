use actix_web::{get, web};
use utoipa::openapi::{self, Server};

use crate::{AppState, URLS, enums::ProjectIdentifier};

#[get("")]
#[tracing::instrument(skip(state))]
pub async fn get_openapi(state: web::Data<AppState>) -> web::Json<openapi::OpenApi> {
    let mut openapi = state.openapi.to_owned();
    openapi.servers = Some(vec![Server::new(&URLS.ogc_api.base)]);
    web::Json(openapi)
}

#[get("")]
#[tracing::instrument(skip(state))]
pub async fn get_project_openapi(
    state: web::Data<AppState>,
    project: web::Path<ProjectIdentifier>,
) -> web::Json<openapi::OpenApi> {
    let mut openapi = state.openapi.to_owned();
    openapi.servers = Some(vec![Server::new(format!(
        "{}{}/{}",
        URLS.ogc_api.base, URLS.ogc_api.project, project
    ))]);
    web::Json(openapi)
}
