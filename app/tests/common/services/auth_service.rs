use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct SessionToken(pub String);

pub trait AuthService {
    async fn get_test_session_token(&self, client: &reqwest::Client, user_id: &str)
    -> SessionToken;
}
