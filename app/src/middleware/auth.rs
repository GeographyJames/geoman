use actix_web::{
    HttpMessage,
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    error::{ErrorInternalServerError, ErrorUnauthorized},
    http::header::AUTHORIZATION,
    middleware::Next,
    web::Data,
};

use clerk_rs::validators::{
    authorizer::{ClerkAuthorizer, ClerkError},
    jwks::MemoryCacheJwksProvider,
};

use domain::UserId;
use secrecy::SecretBox;

use crate::{helpers::hash_api_key, postgres::PostgresRepo};

pub async fn dual_auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    let auth_header = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| ErrorUnauthorized("Missing authorisation header"))?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| ErrorUnauthorized("Invalid authorisation format"))?
        .to_string();

    match token.starts_with("gman_") {
        true => validate_api_key(req, next, &SecretBox::new(Box::new(token))).await,
        false => validate_clerk_auth(req, next).await,
    }
}

async fn validate_api_key<B: MessageBody>(
    req: ServiceRequest,
    next: Next<B>,
    token: &secrecy::SecretBox<String>,
) -> Result<ServiceResponse<B>, actix_web::Error> {
    let repo = req
        .app_data::<Data<PostgresRepo>>()
        .ok_or_else(|| ErrorInternalServerError("Database not configured"))?;
    let key_hash = hash_api_key(token);
    let user_id: UserId = repo
        .select_one(&key_hash)
        .await
        .map_err(ErrorInternalServerError)?
        .ok_or_else(|| ErrorUnauthorized("Invalid key"))?;

    req.extensions_mut().insert(user_id);
    next.call(req).await
}

async fn validate_clerk_auth<B: MessageBody>(
    req: ServiceRequest,
    next: Next<B>,
) -> Result<ServiceResponse<B>, actix_web::Error> {
    let clerk_authoriser = req
        .app_data::<Data<ClerkAuthorizer<MemoryCacheJwksProvider>>>()
        .ok_or_else(|| ErrorInternalServerError("Clerk authoriser not configured"))?;
    match clerk_authoriser.authorize(&req).await {
        // We have authed request and can pass the user onto the next body
        Ok(jwt) => {
            req.extensions_mut().insert(jwt);
            next.call(req).await
        }
        // Output any other errors thrown from the Clerk authorizer
        Err(error) => match error {
            ClerkError::Unauthorized(msg) => Err(ErrorUnauthorized(msg)),
            ClerkError::InternalServerError(msg) => Err(ErrorInternalServerError(msg)),
        },
    }
}
