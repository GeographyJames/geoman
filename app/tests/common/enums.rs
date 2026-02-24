use app::AuthenticatedUser;

use crate::common::services::SessionToken;

pub enum Auth {
    Key(String),
    Token(SessionToken),
    MockToken(String),
    MockUserCredentials(AuthenticatedUser),
}

impl Auth {
    pub fn mock_session_token() -> Auth {
        Auth::MockToken(uuid::Uuid::new_v4().to_string())
    }
}
