use crate::{handlers::ApiError, helpers::hash_api_key, postgres::PostgresRepo};
use actix_web::{HttpResponse, post, web};
use anyhow::Context;
use clerk_rs::validators::authorizer::ClerkJwt;
use domain::{ApiKeyInputDTO, UserId};
use rand::{Rng, distr::Alphanumeric};
use secrecy::{ExposeSecret, SecretBox};
use serde::{Deserialize, Serialize};

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
    let user_id: UserId = repo
        .select_one(&*jwt)
        .await?
        .ok_or_else(|| ApiError::Unexpected(anyhow::anyhow!("Api key not found in database")))?;
    let api_key = generate_api_key_string();
    let key_hash = hash_api_key(&api_key);
    let RequestPayload { key_name } = payload.into_inner();
    let key = ApiKeyInputDTO {
        user_id,
        name: key_name,
        key_hash,
    };
    let _key_id = repo
        .insert(&key)
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
