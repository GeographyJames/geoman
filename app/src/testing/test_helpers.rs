use actix_web::{
    App,
    body::MessageBody,
    dev::{HttpServiceFactory, ServiceResponse},
    middleware::{self},
    test::{self, TestRequest},
    web::{Data, scope},
};

use clerk_rs::{ClerkConfiguration, clerk::Clerk};
use serde_json::json;

use crate::{
    MockUserCredentials, middleware::mock_auth_middlewear, postgres::PostgresRepo,
    types::UserClient,
};

pub async fn mock_app(
    service: impl HttpServiceFactory + 'static,
    req: TestRequest,
    user: MockUserCredentials,
) -> ServiceResponse<impl MessageBody> {
    let repo = PostgresRepo::mock();
    let clerk_config = ClerkConfiguration::new(None, None, None, None);
    let app = test::init_service(
        App::new()
            .app_data(Data::new(repo))
            .app_data(Data::new(UserClient(Clerk::new(clerk_config))))
            .wrap(middleware::from_fn(mock_auth_middlewear))
            .service(scope("/").service(service)),
    )
    .await;
    let req = req
        .insert_header(("x-test-user", json!(user).to_string()))
        .to_request();

    test::call_service(&app, req).await
}
