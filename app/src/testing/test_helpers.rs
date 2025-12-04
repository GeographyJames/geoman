use actix_http::Request;
use actix_web::{
    App,
    body::MessageBody,
    dev::{HttpServiceFactory, ServiceResponse},
    middleware::{self},
    test,
    web::{Data, scope},
};

use crate::{middleware::mock_auth_middlewear, postgres::PostgresRepo};

pub async fn mock_app(
    repo: PostgresRepo,
    service: impl HttpServiceFactory + 'static,
    req: Request,
) -> ServiceResponse<impl MessageBody> {
    let app = test::init_service(
        App::new()
            .app_data(Data::new(repo))
            .wrap(middleware::from_fn(mock_auth_middlewear))
            .service(scope("/").service(service)),
    )
    .await;
    test::call_service(&app, req).await
}
