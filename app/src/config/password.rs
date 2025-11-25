use secrecy::{CloneableSecret, ExposeSecret, zeroize::Zeroize};
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Password(String);
impl Zeroize for Password {
    fn zeroize(&mut self) {
        self.0.zeroize();
    }
}
impl CloneableSecret for Password {}
impl ExposeSecret<String> for Password {
    fn expose_secret(&self) -> &String {
        &self.0
    }
}
impl Password {
    pub fn new(password: String) -> Self {
        Self(password)
    }
}
