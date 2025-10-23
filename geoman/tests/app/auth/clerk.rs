use secrecy::{ExposeSecret, SecretBox};
use serde::Deserialize;

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

impl ClerkAuthProvider {
    pub async fn get_test_session_token(&self, client: &reqwest::Client) -> String {
        let response = client
            .post("https://api.clerk.com/v1/sessions")
            .header("Content-Type", "application/json")
            .header(
                "Authorization",
                format!("Bearer {}", self.secret.expose_secret().clone()),
            )
            .json(&serde_json::json!({
                "user_id": self.user_id
            }))
            .send()
            .await
            .expect("failed to execute request for Clerk session");

        let session: ClerkSession = response.json().await.expect("failed to deserialise json");
        let response = client
            .post(format!(
                "https://api.clerk.com/v1/sessions/{}/tokens",
                session.id
            ))
            .header("Content-Type", "application/json")
            .header(
                "Authorization",
                format!("Bearer {}", self.secret.expose_secret().clone()),
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
