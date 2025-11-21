use std::fmt::Display;

use serde::Deserialize;
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
pub struct ProjectFeatureId {
    pub collection_id: ProjectCollectionId,
    pub id: i32,
}
#[derive(Clone, Copy, Default, Debug)]
pub struct ProjectId(pub i32);
#[derive(Clone, Copy, Default, Debug)]
pub struct UserId(pub i32);
#[derive(Clone, Copy, Default, Debug)]
pub struct TeamId(pub i32);

#[derive(Clone, Copy, Default, Debug, Deserialize)]
pub struct ProjectCollectionId(pub i32);

impl Display for ProjectCollectionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for ProjectFeatureId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "collection id: {}, feature id: {}",
            self.collection_id, self.id
        )
    }
}
