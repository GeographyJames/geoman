use actix_web::{
    HttpMessage,
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
};
use domain::{TeamId, UserId};
use serde::{Deserialize, Serialize};

use crate::types::AuthenticatedUser;

#[derive(Serialize, Deserialize)]
pub enum MockUserCredentials {
    Token(String),
    User(AuthenticatedUser),
}

pub async fn mock_auth_middlewear(
    req: ServiceRequest,
    next: Next<impl actix_web::body::MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    if let Some(user) = req
        .headers()
        .get("X-Test-User")
        .and_then(|h| h.to_str().ok())
        .and_then(|str| serde_json::from_str(str).ok())
        .and_then(|json| serde_json::from_value::<MockUserCredentials>(json).ok())
    {
        let user = match user {
            MockUserCredentials::Token(_) => AuthenticatedUser {
                id: UserId(0),
                first_name: uuid::Uuid::new_v4().to_string(),
                last_name: uuid::Uuid::new_v4().to_string(),
                username: None,
                team_id: TeamId(-1),
                admin: false,
            },
            MockUserCredentials::User(authenticated_user) => authenticated_user,
        };
        req.extensions_mut().insert(user);
    }

    next.call(req).await
}
