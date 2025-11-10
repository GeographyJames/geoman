use actix_web::{HttpResponse, web};
use utoipa::openapi::OpenApi;

#[tracing::instrument(skip(api_docs))]
pub async fn get_api_docs(api_docs: web::Data<OpenApi>) -> HttpResponse {
    let openapi = api_docs.as_ref();
    HttpResponse::Ok().json(openapi)
}
