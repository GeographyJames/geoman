use actix_web::{
    App,
    body::MessageBody,
    dev::{HttpServiceFactory, ServiceResponse},
    middleware::{self},
    test::{self, TestRequest},
    web::{Data, scope},
};

use serde_json::json;

use crate::{MockUserCredentials, middleware::mock_auth_middlewear, postgres::PostgresRepo};

pub async fn mock_app(
    service: impl HttpServiceFactory + 'static,
    req: TestRequest,
    user: MockUserCredentials,
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
        .insert_header(("x-test-user", json!(user).to_string()))
        .to_request();

    test::call_service(&app, req).await
}

/// Like `mock_app`, but registers the service without a scope wrapper.
/// Use for handlers with path parameters (e.g. `#[patch("/{id}")]`), where
/// `scope("/")` would strip the leading slash and prevent route matching.
pub async fn mock_app_with_path_params(
    service: impl HttpServiceFactory + 'static,
    req: TestRequest,
    user: MockUserCredentials,
) -> ServiceResponse<impl MessageBody> {
    let repo = PostgresRepo::mock();
    let app = test::init_service(
        App::new()
            .app_data(Data::new(repo))
            .wrap(middleware::from_fn(mock_auth_middlewear))
            .service(service),
    )
    .await;
    let req = req
        .insert_header(("x-test-user", json!(user).to_string()))
        .to_request();

    test::call_service(&app, req).await
}
