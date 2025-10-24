use dotenvy::dotenv;
use reqwest::Response;
use secrecy::{ExposeSecret, SecretBox};
use serde::{Deserialize, de::DeserializeOwned};
use tokio::sync::OnceCell;

use crate::constants::CLERK_USER_ID_KEY;

static TEST_SESSION: OnceCell<ClerkSession> = OnceCell::const_new();

#[derive(Deserialize)]
struct ClerkSession {
    id: String,
}

#[derive(Deserialize)]
pub struct ClerkSessionToken {
    pub jwt: String,
}

pub struct ClerkAuthProvider {
    pub secret: SecretBox<String>,
}

async fn handle_json_response<T: DeserializeOwned>(response: Response) -> Result<T, String> {
    if response.status().is_success() {
        let token: T = response
            .json()
            .await
            .expect("failed to deserialise successful Clerk response");
        return Ok(token);
    }
    let status = response.status().as_u16();
    let error: serde_json::Value = response
        .json()
        .await
        .unwrap_or(serde_json::json!("failed to deserialise Clerk error"));
    Err(format!("Clerk responded with status: {status}, {error}",))
}

async fn get_session(client: &reqwest::Client, secret: &SecretBox<String>) -> ClerkSession {
    dotenv().ok();
    let user_id = std::env::var(CLERK_USER_ID_KEY)
        .expect(&format!("no {CLERK_USER_ID_KEY} environment variable set"));

    let response = client
        .post("https://api.clerk.com/v1/sessions")
        .header("Content-Type", "application/json")
        .header(
            "Authorization",
            format!("Bearer {}", secret.expose_secret()),
        )
        .json(&serde_json::json!({
            "user_id": user_id
        }))
        .send()
        .await
        .expect("failed to execute request for Clerk session");

    handle_json_response(response)
        .await
        .expect("failed to create Clerk session")
}

impl ClerkAuthProvider {
    pub async fn get_test_session_token(&self, client: &reqwest::Client) -> String {
        let session = TEST_SESSION
            .get_or_init(|| async { get_session(client, &self.secret).await })
            .await;
        let response = client
            .post(format!(
                "https://api.clerk.com/v1/sessions/{}/tokens",
                session.id
            ))
            .header("Content-Type", "application/json")
            .header(
                "Authorization",
                format!("Bearer {}", self.secret.expose_secret()),
            )
            .json(&serde_json::json!({
                "expires_in_seconds": null
            }))
            .send()
            .await
            .expect("failed to execute request for Clerk session token");

        let session_token: ClerkSessionToken = handle_json_response(response)
            .await
            .expect("failed to retrieve Clerk session token");

        session_token.jwt
    }
}
