use std::{net::IpAddr, str::FromStr};

use actix_web::{
    HttpMessage,
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    error::{ErrorInternalServerError, ErrorUnauthorized},
    http::header::{AUTHORIZATION, USER_AGENT},
    middleware::Next,
    web::Data,
};

use anyhow::Context;
use clerk_rs::validators::{
    authorizer::{ClerkAuthorizer, ClerkError},
    jwks::MemoryCacheJwksProvider,
};

use domain::UserInputDto;
use secrecy::SecretBox;

use crate::{
    helpers::hash_api_key, postgres::PostgresRepo, repo::user_id::SelectOneParams,
    types::AuthenticatedUser,
};

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

pub async fn auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    validate_clerk_auth(req, next).await
}

async fn validate_api_key<B: MessageBody>(
    req: ServiceRequest,
    next: Next<B>,
    token: &secrecy::SecretBox<String>,
) -> Result<ServiceResponse<B>, actix_web::Error> {
    let repo = req
        .app_data::<Data<PostgresRepo>>()
        .ok_or_else(|| ErrorInternalServerError("Postgres Repo not configured"))?;
    let key_hash = hash_api_key(token);
    let ip_address = req
        .connection_info()
        .realip_remote_addr()
        .and_then(|s| IpAddr::from_str(s).ok());
    let user_agent = req
        .headers()
        .get(USER_AGENT)
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    let params = SelectOneParams {
        ip_address,
        user_agent,
    };
    let user: AuthenticatedUser = repo
        .select_one_with_params(&key_hash, &params)
        .await
        .map_err(ErrorInternalServerError)?
        .ok_or_else(|| ErrorUnauthorized("Invalid key"))?;

    req.extensions_mut().insert(user);
    next.call(req).await
}

async fn validate_clerk_auth<B: MessageBody>(
    req: ServiceRequest,
    next: Next<B>,
) -> Result<ServiceResponse<B>, actix_web::Error> {
    let clerk_authoriser = req
        .app_data::<Data<ClerkAuthorizer<MemoryCacheJwksProvider>>>()
        .ok_or_else(|| ErrorInternalServerError("Clerk authoriser not configured"))?;

    let repo = req
        .app_data::<Data<PostgresRepo>>()
        .ok_or_else(|| ErrorInternalServerError("Postgres repo not configured"))?;

    let jwt = match clerk_authoriser.authorize(&req).await {
        Err(error) => match error {
            ClerkError::Unauthorized(msg) => return Err(ErrorUnauthorized(msg)),
            ClerkError::InternalServerError(msg) => return Err(ErrorInternalServerError(msg)),
        },
        Ok(jwt) => jwt,
    };

    let input_dto = UserInputDto {
        auth_id: jwt.sub.as_str(),
        first_name: jwt.other.get("first_name").and_then(|v| v.as_str()),
        last_name: jwt.other.get("last_name").and_then(|v| v.as_str()),
        username: jwt.other.get("username").and_then(|v| v.as_str()),
    };

    let user: AuthenticatedUser = match repo
        .select_one::<AuthenticatedUser, _>(jwt.sub.as_str())
        .await
        .context("failed to retrive user from database")
        .map_err(ErrorInternalServerError)?
    {
        // User found, check the details
        Some(user) => check_user_details(&input_dto, user, repo).await?,
        // User not found in databse, provision
        None => repo
            .insert(&input_dto)
            .await
            .context("failed insert new user do database")
            .map_err(ErrorInternalServerError)?,
    };
    req.extensions_mut().insert(user);
    next.call(req).await
}

async fn check_user_details<'a>(
    input_dto: &UserInputDto<'a>,
    user: AuthenticatedUser,
    repo: &PostgresRepo,
) -> Result<AuthenticatedUser, actix_web::Error> {
    let needs_update = input_dto
        .first_name
        .is_some_and(|name| name != user.first_name.as_str())
        || input_dto
            .last_name
            .is_some_and(|name| name != user.last_name.as_str())
        || input_dto.username != user.username.as_deref();
    if !needs_update {
        return Ok(user);
    }
    repo.update(input_dto)
        .await
        .map_err(ErrorInternalServerError)
}
