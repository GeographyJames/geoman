use std::net::TcpListener;

use actix_web::{App, HttpResponse, HttpServer, dev::Server, web};
use actix_web_httpauth::middleware::HttpAuthentication;

use crate::{URLS, app::middleware};

pub fn run(listner: TcpListener) -> anyhow::Result<Server> {
    let server = HttpServer::new(move || {
        let auth = HttpAuthentication::bearer(middleware::auth::validataor);
        App::new()
            .route(&URLS.health_check, web::get().to(HttpResponse::Ok))
            .service(
                web::resource(&URLS.health_check_authenticated)
                    .route(web::get().to(HttpResponse::Ok))
                    .wrap(auth)
            )
    })
    .listen(listner)?
    .run();
    Ok(server)
}
