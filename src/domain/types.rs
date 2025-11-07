use derive_more::AsRef;
use sqlx::prelude::Type;

#[derive(Type, Clone, AsRef)]
#[sqlx(transparent)]
#[sqlx(type_name = "slug")]
pub struct Slug(String);
impl Slug {
    pub fn slugify(value: &str) -> Self {
        Slug(slug::slugify(value))
    }
    pub fn parse(value: String) -> Result<Self, String> {
        let slug = slug::slugify(&value);
        if slug == value {
            Ok(Self(value))
        } else {
            Err("value is not a slug".to_string())
        }
    }
}
