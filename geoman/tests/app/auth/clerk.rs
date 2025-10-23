use secrecy::{ExposeSecret, SecretBox};
use serde::Deserialize;
use tokio::sync::OnceCell;

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
    pub user_id: String,
}

async fn get_session(
    client: &reqwest::Client,
    secret: &SecretBox<String>,
    user_id: &str,
) -> ClerkSession {
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

    response.json().await.expect("failed to deserialise json")
}

impl ClerkAuthProvider {
    pub async fn get_test_session_token(&self, client: &reqwest::Client) -> String {
        let session = TEST_SESSION
            .get_or_init(|| async { get_session(client, &self.secret, &self.user_id).await })
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
        let session_token: ClerkSessionToken =
            response.json().await.expect("failed to deserialise json");
        session_token.jwt
    }
}
