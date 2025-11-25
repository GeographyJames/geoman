use secrecy::SecretBox;
use serde::Deserialize;

/// Clerk authentication settings
#[derive(Deserialize)]
pub struct ClerkAuthSettings {
    pub clerk_secret_key: SecretBox<String>,
}
