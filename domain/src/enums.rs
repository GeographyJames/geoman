use serde::{Deserialize, Deserializer, Serialize};
use sqlx::prelude::Type;

use crate::ProjectCollectionId;

#[derive(Serialize, Deserialize)]
pub enum Status {
    Active,
    Archived,
    Deleted,
}

#[derive(Debug, Clone, Type, Serialize, Deserialize)]
#[sqlx(type_name = "geometry_type", rename_all = "UPPERCASE")]
pub enum GeometryType {
    Point,
    LineString,
    Polygon,
    MultiPoint,
    MultiLineString,
    MultiPolygon,
    GeometryCollection,
}

#[derive(Clone, Debug)]
pub enum CollectionId {
    Projects,
    ProjectCollection(ProjectCollectionId),
    Other(String),
}

impl From<ProjectCollectionId> for CollectionId {
    fn from(value: ProjectCollectionId) -> Self {
        Self::ProjectCollection(value)
    }
}

impl std::fmt::Display for CollectionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            CollectionId::Projects => "projects".to_string(),
            CollectionId::ProjectCollection(id) => id.to_string(),
            CollectionId::Other(s) => s.clone(),
        };
        write!(f, "{}", s)
    }
}

impl<'de> Deserialize<'de> for CollectionId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(s.into())
    }
}

impl From<String> for CollectionId {
    fn from(s: String) -> Self {
        if let Ok(id) = s.parse::<i32>() {
            return CollectionId::ProjectCollection(ProjectCollectionId(id));
        }

        match s.to_lowercase().as_str() {
            "projects" => CollectionId::Projects,
            _ => CollectionId::Other(s),
        }
    }
}
