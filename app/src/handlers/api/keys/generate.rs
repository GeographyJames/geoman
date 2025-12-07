use crate::{
    handlers::ApiError,
    helpers::{get_user_context, hash_api_key},
    postgres::PostgresRepo,
    types::{AuthenticatedUser, UserClient},
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
#[tracing::instrument(skip(repo, payload, user, user_client))]
pub async fn generate_api_key(
    user: web::ReqData<AuthenticatedUser>,
    repo: web::Data<PostgresRepo>,
    payload: web::Json<ApiKeyReqPayload>,
    user_client: web::Data<UserClient>,
) -> Result<HttpResponse, ApiError> {
    let user_context = get_user_context(&repo, user.into_inner(), &user_client).await?;
    let api_key = generate_api_key_string();
    let key_hash = hash_api_key(&api_key);
    let ApiKeyReqPayload { key_name } = payload.into_inner();
    let key = ApiKeyInputDTO {
        user_id: user_context.id,
        name: key_name,
        key_hash,
    };
    let key_id = repo
        .insert(&key)
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
