use actix_web::{HttpResponse, web};
use utoipa::openapi::OpenApi;

pub async fn get_api_docs(api_docs: web::Data<OpenApi>) -> HttpResponse {
    HttpResponse::Ok().json(api_docs)
}
