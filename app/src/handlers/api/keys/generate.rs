use crate::{
    handlers::ApiError, helpers::hash_api_key, postgres::PostgresRepo, types::AuthenticatedUser,
};
use actix_web::{HttpResponse, post, web};
use anyhow::Context;
use domain::{ApiKeyInputDTO, KeyId};
use rand::{Rng, distr::Alphanumeric};
use secrecy::{ExposeSecret, SecretBox};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ApiKeyReqPayload {
    pub key_name: String,
}
#[derive(Serialize, Deserialize)]
pub struct ApiKeyResPayload {
    pub api_key: String,
    pub id: KeyId,
}

#[post("")]
#[tracing::instrument(skip(repo, payload, user))]
pub async fn generate_api_key(
    user: web::ReqData<AuthenticatedUser>,
    repo: web::Data<PostgresRepo>,
    payload: web::Json<ApiKeyReqPayload>,
) -> Result<HttpResponse, ApiError> {
    let auth_id = match user.into_inner() {
        AuthenticatedUser::AuthenticationId(id) => id,
        AuthenticatedUser::User(_) => {
            return Err(ApiError::Unexpected(anyhow::anyhow!(
                "Expected AuthenticationId, got User context"
            )));
        }
    };
    let api_key = generate_api_key_string();
    let key_hash = hash_api_key(&api_key);
    let ApiKeyReqPayload { key_name } = payload.into_inner();
    let key = ApiKeyInputDTO {
        name: key_name,
        key_hash,
    };
    let key_id = repo
        .insert(&(key, auth_id.as_str()))
        .await
        .context("failed to save key in database")?;

    Ok(HttpResponse::Ok().json(ApiKeyResPayload {
        id: key_id,
        api_key: api_key.expose_secret().clone(),
    }))
}

fn generate_api_key_string() -> SecretBox<String> {
    let mut rng = rand::rng();
    let random_part: String = (0..32).map(|_| rng.sample(Alphanumeric) as char).collect();
    SecretBox::new(Box::new(format!("gman_{}", random_part)))
}
