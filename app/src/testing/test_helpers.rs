use actix_http::Request;
use actix_web::{
    App, HttpMessage,
    body::MessageBody,
    dev::{HttpServiceFactory, ServiceRequest, ServiceResponse},
    middleware::{self, Next},
    test,
    web::{Data, scope},
};
use domain::UserId;

use crate::postgres::PostgresRepo;

pub async fn mock_auth_middlewear(
    req: ServiceRequest,
    next: Next<impl actix_web::body::MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    req.extensions_mut().insert(UserId(0));
    next.call(req).await
}

pub async fn mock_app(
    service: impl HttpServiceFactory + 'static,
    req: Request,
) -> ServiceResponse<impl MessageBody> {
    let mock_repo = PostgresRepo::mock();
    let app = test::init_service(
        App::new()
            .app_data(Data::new(mock_repo))
            .wrap(middleware::from_fn(mock_auth_middlewear))
            .service(scope("/").service(service)),
    )
    .await;
    test::call_service(&app, req).await
}
