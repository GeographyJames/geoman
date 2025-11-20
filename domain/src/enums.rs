use serde::{Deserialize, Deserializer, Serialize};
use sqlx::prelude::Type;

use crate::Slug;

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
pub enum Collection {
    Projects,
    Other(String),
}

impl std::fmt::Display for Collection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Collection::Projects => "projects".to_string(),
            Collection::Other(s) => s.clone(),
        };
        write!(f, "{}", s)
    }
}

impl<'de> Deserialize<'de> for Collection {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.to_lowercase().as_str() {
            "projects" => Ok(Collection::Projects),
            _ => Ok(Collection::Other(s)),
        }
    }
}

impl TryInto<Slug> for Collection {
    type Error = String;

    fn try_into(self) -> Result<Slug, Self::Error> {
        Slug::parse(self.to_string())
    }
}
