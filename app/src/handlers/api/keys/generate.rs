use crate::{handlers::ApiError, postgres::PostgresRepo};
use actix_web::{HttpResponse, post, web};
use anyhow::Context;

use clerk_rs::validators::authorizer::ClerkJwt;
use rand::{Rng, distr::Alphanumeric};
use secrecy::{ExposeSecret, SecretBox};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Deserialize, Serialize)]
pub struct RequestPayload {
    pub key_name: String,
}
#[derive(Serialize, Deserialize)]
pub struct ResponsePayload {
    pub api_key: String,
}

#[post("")]
#[tracing::instrument(skip(jwt, repo, payload))]
pub async fn generate_api_key(
    jwt: web::ReqData<ClerkJwt>,
    repo: web::Data<PostgresRepo>,
    payload: web::Json<RequestPayload>,
) -> Result<HttpResponse, ApiError> {
    let clerk_user_id = &jwt.sub;
    let user_id = sqlx::query_scalar!(
        "SELECT id FROM app.users WHERE clerk_id = $1",
        clerk_user_id
    )
    .fetch_optional(&repo.db_pool)
    .await
    .context("failed to query database for user")?
    .ok_or_else(|| ApiError::Authentication)?;
    let api_key = generate_api_key_string();
    let api_key_hash = hash_api_key(&api_key);

    sqlx::query!(
        "INSERT INTO app.api_keys (user_id, name, key_hash) VALUES ($1, $2, $3)",
        user_id,
        payload.key_name,
        api_key_hash
    )
    .execute(&repo.db_pool)
    .await
    .context("failed to save key in database")?;

    Ok(HttpResponse::Ok().json(ResponsePayload {
        api_key: api_key.expose_secret().clone(),
    }))
}

fn generate_api_key_string() -> SecretBox<String> {
    let mut rng = rand::rng();
    let random_part: String = (0..32).map(|_| rng.sample(Alphanumeric) as char).collect();
    SecretBox::new(Box::new(format!("gman_{}", random_part)))
}

/// Hash an API key using SHA256 for database storage
fn hash_api_key(api_key: &SecretBox<String>) -> String {
    let mut hasher = Sha256::new();
    hasher.update(api_key.expose_secret().as_bytes());
    hex::encode(hasher.finalize())
}
