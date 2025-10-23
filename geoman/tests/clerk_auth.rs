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

pub async fn get_test_session(
    clerk_secret: &SecretBox<String>,
    clerk_user_id: &str,
) -> ClerkSessionToken {
    let client = reqwest::Client::new();
    let response = client
        .post("https://api.clerk.com/v1/sessions")
        .header("Content-Type", "application/json")
        .header(
            "Authorization",
            format!("Bearer {}", clerk_secret.expose_secret().clone()),
        )
        .json(&serde_json::json!({
            "user_id": clerk_user_id
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
            format!("Bearer {}", clerk_secret.expose_secret().clone()),
        )
        .json(&serde_json::json!({
            "expires_in_seconds": null
        }))
        .send()
        .await
        .expect("failed to execute request for Clerk session token");
    response.json().await.expect("failed to deserialise json")
}
