use crate::common::services::SessionToken;

pub enum Auth {
    Key(String),
    Token(SessionToken),
    MockToken(String),
}

impl Auth {
    pub fn mock_session_token() -> Auth {
        Auth::MockToken("test_user_123".to_string())
    }
}
