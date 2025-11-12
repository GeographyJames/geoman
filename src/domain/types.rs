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

impl Default for Slug {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4().to_string())
    }
}

#[derive(Clone, Copy, Default)]
pub struct ProjectId(pub i32);
#[derive(Clone, Copy, Default)]
pub struct UserId(pub i32);
#[derive(Clone, Copy, Default)]
pub struct TeamId(pub i32);
#[derive(Clone, Copy, Default)]
pub struct FeatureId(pub i32);

#[derive(Clone, Copy, Default)]
pub struct CollectionId(pub i32);
