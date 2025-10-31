use actix_web::HttpResponse;

#[tracing::instrument]
pub async fn get_projects() -> HttpResponse {
    HttpResponse::Ok().finish()
}
