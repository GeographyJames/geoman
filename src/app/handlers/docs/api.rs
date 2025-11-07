use actix_web::{HttpRequest, HttpResponse, web};
use utoipa::openapi::{OpenApi, Server};

pub async fn get_api_docs(req: HttpRequest, api_docs: web::Data<OpenApi>) -> HttpResponse {
    let mut openapi = api_docs.as_ref().clone();

    // Add servers field dynamically based on request
    let connection_info = req.connection_info();
    let base_url = format!("{}://{}/ogcapi", connection_info.scheme(), connection_info.host());
    openapi.servers = Some(vec![Server::new(base_url)]);

    HttpResponse::Ok().json(openapi)
}
