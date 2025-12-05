use serde::Deserialize;

use crate::Password;

/// Clerk authentication settings
#[derive(Deserialize, Clone)]
pub struct ClerkAuthSettings {
    pub clerk_secret_key: Password,
}
