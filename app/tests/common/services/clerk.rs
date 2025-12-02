use secrecy::{ExposeSecret, SecretBox};
use serde::Deserialize;

use crate::common::{
    helpers::handle_json_response,
    services::{AuthService, auth_service::SessionToken},
};

#[derive(Deserialize, Debug)]
struct ClerkSession {
    id: String,
}

pub struct ClerkAuthService {
    pub secret: SecretBox<String>,
}

#[derive(Deserialize, Debug, Clone)]
struct ClerkSessionToken {
    pub jwt: String,
}

impl ClerkAuthService {
    async fn get_session(&self, client: &reqwest::Client, clerk_user_id: &str) -> ClerkSession {
        let response = client
            .post("https://api.clerk.com/v1/sessions")
            .header("Content-Type", "application/json")
            .header(
                "Authorization",
                format!("Bearer {}", self.secret.expose_secret()),
            )
            .json(&serde_json::json!({
                "user_id": clerk_user_id
            }))
            .send()
            .await
            .expect("failed to execute request for Clerk session");

        handle_json_response(response)
            .await
            .expect("failed to create Clerk session")
    }
}

impl AuthService for ClerkAuthService {
    async fn get_test_session_token(
        &self,
        client: &reqwest::Client,
        clerk_user_id: &str,
    ) -> SessionToken {
        let session = self.get_session(client, clerk_user_id).await;
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

        SessionToken(session_token.jwt)
    }
}
