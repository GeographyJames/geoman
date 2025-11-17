use sqlx::prelude::Type;

#[derive(Type, Clone)]
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
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl Default for Slug {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4().to_string())
    }
}

impl AsRef<str> for Slug {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub struct ProjectId(pub i32);
#[derive(Clone, Copy, Default)]
pub struct UserId(pub i32);
#[derive(Clone, Copy, Default)]
pub struct TeamId(pub i32);
#[derive(Clone, Copy, Default)]
pub struct FeatureId(pub i32);

#[derive(Clone, Copy, Default)]
pub struct CollectionId(pub i32);
