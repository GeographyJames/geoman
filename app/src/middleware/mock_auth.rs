use actix_web::{
    HttpMessage,
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
};
use domain::UserId;

pub async fn mock_auth_middlewear(
    req: ServiceRequest,
    next: Next<impl actix_web::body::MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    req.extensions_mut().insert(UserId(0));
    next.call(req).await
}
