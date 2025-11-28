use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct SessionToken {
    pub jwt: String,
}
