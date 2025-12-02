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
use clerk_rs::{
    clerk::Clerk,
    validators::{
        authorizer::{ClerkAuthorizer, ClerkError, ClerkJwt},
        jwks::MemoryCacheJwksProvider,
    },
};

use domain::UserId;
use secrecy::SecretBox;

use crate::{helpers::hash_api_key, postgres::PostgresRepo, repo::user_id::SelectOneParams};

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
        .ok_or_else(|| ErrorInternalServerError("Database not configured"))?;
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
    let user_id: UserId = repo
        .select_one_with_params(&key_hash, &params)
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

    let repo = req
        .app_data::<Data<PostgresRepo>>()
        .ok_or_else(|| ErrorInternalServerError("Database not configured"))?;

    match clerk_authoriser.authorize(&req).await {
        // We have authed request and can pass the user onto the next body
        Ok(jwt) => {
            // Look up the user in the database
            let user_id: UserId = match repo
                .select_one(&jwt)
                .await
                .map_err(ErrorInternalServerError)?
            {
                Some(user) => user,
                None => {
                    tracing::info!("User not found in database: provisioning");
                    let clerk = req
                        .app_data::<Data<Clerk>>()
                        .ok_or_else(|| ErrorInternalServerError("Clerk not conifigured"))?;

                    provision_clerk_user(repo, jwt, clerk)
                        .await
                        .context("failed to provision new user")
                        .map_err(ErrorInternalServerError)?
                }
            };

            req.extensions_mut().insert(user_id);
            next.call(req).await
        }
        // Output any other errors thrown from the Clerk authorizer
        Err(error) => match error {
            ClerkError::Unauthorized(msg) => Err(ErrorUnauthorized(msg)),
            ClerkError::InternalServerError(msg) => Err(ErrorInternalServerError(msg)),
        },
    }
}

async fn provision_clerk_user(
    repo: &PostgresRepo,
    jwt: ClerkJwt,
    clerk_client: &Clerk,
) -> Result<UserId, anyhow::Error> {
    let user = clerk_rs::apis::users_api::User::get_user(clerk_client, &jwt.sub)
        .await
        .context("failed to retrive user from Clerk")?;
    sqlx::query_scalar!(
        r#"INSERT INTO app.users (
            clerk_id, first_name, last_name
            ) VALUES ($1, $2, $3)
             RETURNING id AS "id: UserId""#,
        jwt.sub,
        user.first_name
            .ok_or_else(|| anyhow::anyhow!("User has no first name"))?,
        user.last_name
            .ok_or_else(|| anyhow::anyhow!("User has no last name"))?
    )
    .fetch_one(&repo.db_pool)
    .await
    .context("failed to add new user to databqase")
}
