use actix_web::HttpRequest;
use anyhow::Context;
use domain::{KeyHash, UserInputDto};
use secrecy::{ExposeSecret, SecretBox};
use sha2::{Digest, Sha256};

use crate::{
    constants::CONFIGURATION_DIRECTORY,
    postgres::PostgresRepo,
    types::{AuthenticatedUser, UserClient, UserContext},
};

pub fn get_configuration_directory() -> anyhow::Result<std::path::PathBuf> {
    let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = manifest_dir
        .parent()
        .context("failed to get workspace root")?;

    Ok(workspace_root.join(CONFIGURATION_DIRECTORY))
}

/// Build base URL from request
pub fn get_base_url(req: &HttpRequest) -> String {
    let connection_info = req.connection_info();
    format!("{}://{}", connection_info.scheme(), connection_info.host())
}

pub fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    write!(f, "{}", e)?;
    let mut current = e.source();
    while let Some(cause) = current {
        write!(f, "\ncaused by:\n\t{}", cause)?;
        current = cause.source();
    }
    Ok(())
}

/// Hash an API key using SHA256 for database storage
pub fn hash_api_key(api_key: &SecretBox<String>) -> KeyHash {
    let mut hasher = Sha256::new();
    hasher.update(api_key.expose_secret().as_bytes());
    KeyHash(hex::encode(hasher.finalize()))
}

pub async fn get_user_context(
    repo: &PostgresRepo,
    user: AuthenticatedUser,
    user_client: &UserClient,
) -> Result<UserContext, anyhow::Error> {
    Ok(match user {
        AuthenticatedUser::AuthenticationId(id) => match repo
            .select_one(id.as_str())
            .await
            .context("failed to retrieve use from database...")?
        {
            Some(user_context) => user_context,
            None => {
                tracing::info!("User not found in database, provisioning new user");
                provision_user(repo, id, user_client)
                    .await
                    .context("failed to provision new user")?
            }
        },
        AuthenticatedUser::User(user_context) => user_context,
    })
}

async fn provision_user(
    repo: &PostgresRepo,
    id: String,
    user_client: &UserClient,
) -> Result<UserContext, anyhow::Error> {
    let user = clerk_rs::apis::users_api::User::get_user(&user_client.0, &id)
        .await
        .context("failed to retrieve user from Clerk")?;

    let first_name = user.first_name.flatten().unwrap_or("Unknown".to_string());
    let last_name = user.last_name.flatten().unwrap_or("User".to_string());
    let new_user = UserInputDto {
        auth_id: id,
        first_name,
        last_name,
    };
    repo.insert(&new_user)
        .await
        .context("failed to insert new use to database")
}
