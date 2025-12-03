use serde::Serialize;
use slug::slugify;

#[derive(Debug, Serialize)]
pub struct ProjectSlug(String);

impl ProjectSlug {
    pub fn parse(s: &str) -> Self {
        Self(slugify(s))
    }
}

impl AsRef<str> for ProjectSlug {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
