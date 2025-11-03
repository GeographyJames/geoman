use actix_web::{HttpResponse, get};
use serde::Serialize;

#[derive(Serialize, utoipa::ToSchema)]

pub struct Project {
    id: u32,
    name: String,
}

#[utoipa::path(
    responses((status = OK, body = Vec<Project>)),
)]
#[get("")]
#[tracing::instrument]
pub async fn get_projects() -> HttpResponse {
    let projects: Vec<Project> = Vec::new();
    HttpResponse::Ok().json(projects)
}
