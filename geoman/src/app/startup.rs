use std::net::TcpListener;

use actix_web::{App, HttpResponse, HttpServer, dev::Server, web};

use crate::URLS;

pub fn run(listner: TcpListener) -> anyhow::Result<Server> {
    let server =
        HttpServer::new(|| App::new().route(&URLS.health_check, web::get().to(HttpResponse::Ok)))
            .listen(listner)?
            .run();
    Ok(server)
}
