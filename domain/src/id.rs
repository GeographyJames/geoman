use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Clone, Copy, Default, Debug)]
pub struct ProjectFeatureId {
    pub collection_id: ProjectCollectionId,
    pub id: i32,
}
#[derive(Clone, Copy, Default, Debug, Deserialize, Serialize, sqlx::Type, PartialEq)]
#[sqlx(transparent)]
pub struct ProjectId(pub i32);

#[derive(Clone, Copy, Default, Debug, Deserialize, Serialize, sqlx::Type, PartialEq)]
#[sqlx(transparent)]
pub struct TechnologyId(pub i32);

#[derive(Clone, Copy, Default, Debug, Deserialize, Serialize, sqlx::Type, PartialEq)]
#[sqlx(transparent)]
pub struct SubdivisionId(pub i32);

#[derive(Clone, Copy, Default, Debug, sqlx::Type, Serialize, Deserialize, PartialEq)]
#[sqlx(transparent)]
pub struct UserId(pub i32);
#[derive(Clone, Copy, Default, Debug, Deserialize, Serialize, sqlx::Type, PartialEq)]
#[sqlx(transparent)]
pub struct TeamId(pub i32);

#[derive(Clone, Copy, Default, Debug, Deserialize)]
pub struct FeatureId(pub i32);

#[derive(Clone, Copy, Default, Debug, Deserialize, Serialize, sqlx::Type, PartialEq)]
#[sqlx(transparent)]
pub struct KeyId(pub i32);

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

impl Display for ProjectId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for FeatureId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
