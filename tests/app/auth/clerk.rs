use secrecy::{ExposeSecret, SecretBox};
use serde::Deserialize;
use tokio::sync::OnceCell;

use crate::app::helpers::handle_json_response;

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
    pub test_user_id: String,
}

impl ClerkAuthProvider {
    pub async fn get_test_session_token(&self, client: &reqwest::Client) -> String {
        let session = TEST_SESSION
            .get_or_init(|| async { self.get_session(client).await })
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
            .send()
            .await
            .expect("failed to execute request for Clerk session token");

        let session_token: ClerkSessionToken = handle_json_response(response)
            .await
            .expect("failed to retrieve Clerk session token");

        session_token.jwt
    }

    async fn get_session(&self, client: &reqwest::Client) -> ClerkSession {
        let response = client
            .post("https://api.clerk.com/v1/sessions")
            .header("Content-Type", "application/json")
            .header(
                "Authorization",
                format!("Bearer {}", self.secret.expose_secret()),
            )
            .json(&serde_json::json!({
                "user_id": self.test_user_id
            }))
            .send()
            .await
            .expect("failed to execute request for Clerk session");

        handle_json_response(response)
            .await
            .expect("failed to create Clerk session")
    }
}
