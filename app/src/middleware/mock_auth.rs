use actix_web::{
    HttpMessage,
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
};

use crate::types::AuthenticatedUser;

pub async fn mock_auth_middlewear(
    req: ServiceRequest,
    next: Next<impl actix_web::body::MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    if let Some(user) = req
        .headers()
        .get("X-Test-User")
        .and_then(|h| h.to_str().ok())
        .and_then(|str| serde_json::from_str(str).ok())
        .and_then(|json| serde_json::from_value::<AuthenticatedUser>(json).ok())
    {
        req.extensions_mut().insert(user);
    }

    next.call(req).await
}
