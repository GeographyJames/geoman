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
    let user_id = req
        .headers()
        .get("X-Test-User-Id")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.parse::<i32>().ok())
        .map(UserId)
        .unwrap_or(UserId(0)); // Default to 0

    req.extensions_mut().insert(user_id);
    next.call(req).await
}
