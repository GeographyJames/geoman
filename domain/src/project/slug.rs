use serde::Serialize;
use slug::slugify;

#[derive(Debug, Serialize)]
pub struct ProjectSlugInputDto(String);

impl ProjectSlugInputDto {
    pub fn parse(s: &str) -> Self {
        Self(slugify(s))
    }
}

impl AsRef<str> for ProjectSlugInputDto {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
