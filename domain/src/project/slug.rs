use serde::Serialize;
use slug::slugify;

#[derive(Debug, Serialize)]
pub struct ProjectSlugInputDto(String);

impl AsRef<str> for ProjectSlugInputDto {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for ProjectSlugInputDto {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let slug = slugify(&value);
        if slug != value {
            return Err("invalid slug".to_string());
        }
        Ok(Self(slug))
    }
}

impl Default for ProjectSlugInputDto {
    fn default() -> Self {
        Self(Default::default())
    }
}
