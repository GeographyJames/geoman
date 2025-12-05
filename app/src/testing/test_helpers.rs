use actix_web::{
    App,
    body::MessageBody,
    dev::{HttpServiceFactory, ServiceResponse},
    middleware::{self},
    test::{self, TestRequest},
    web::{Data, scope},
};
use domain::UserId;

use crate::{middleware::mock_auth_middlewear, postgres::PostgresRepo};

pub async fn mock_app(
    service: impl HttpServiceFactory + 'static,
    req: TestRequest,
    user_id: UserId,
) -> ServiceResponse<impl MessageBody> {
    let repo = PostgresRepo::mock();
    let app = test::init_service(
        App::new()
            .app_data(Data::new(repo))
            .wrap(middleware::from_fn(mock_auth_middlewear))
            .service(scope("/").service(service)),
    )
    .await;
    let req = req
        .insert_header(("x-test-user-id", user_id.0.to_string()))
        .to_request();

    test::call_service(&app, req).await
}
