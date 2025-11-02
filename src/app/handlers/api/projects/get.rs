use actix_web::HttpResponse;

use crate::domain::Project;

#[tracing::instrument]
pub async fn get_projects() -> HttpResponse {
    let projects: Vec<Project> = Vec::new();
    HttpResponse::Ok().json(projects)
}
