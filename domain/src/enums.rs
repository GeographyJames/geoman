use crate::{ProjectCollectionId, TableName};
use serde::{Deserialize, Deserializer, Serialize};
use sqlx::prelude::Type;
use strum::Display;

#[derive(Serialize, Deserialize, Debug, Type)]
#[sqlx(type_name = "status", rename_all = "UPPERCASE")]
pub enum Status {
    Active,
    Archived,
    Deleted,
}

#[derive(Serialize, Deserialize, Type, Clone, Debug, Display)]
#[sqlx(type_name = "visibility", rename_all = "UPPERCASE")]
pub enum Visibility {
    Private,
    Team,
    Public,
}

#[derive(Debug, Clone, Type, Serialize, Deserialize, Display)]
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
    DatabaseTable(TableName),
    Other(String),
}

impl From<ProjectCollectionId> for CollectionId {
    fn from(value: ProjectCollectionId) -> Self {
        Self::ProjectCollection(value)
    }
}

impl From<TableName> for CollectionId {
    fn from(value: TableName) -> Self {
        Self::DatabaseTable(value)
    }
}

impl std::fmt::Display for CollectionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            CollectionId::Projects => "projects".to_string(),
            CollectionId::ProjectCollection(id) => id.to_string(),
            CollectionId::DatabaseTable(table) => table.to_string(),
            CollectionId::Other(other) => other.clone(),
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
        if s == "projects" {
            return CollectionId::Projects;
        }
        match TableName::parse(s.clone()) {
            Ok(table_name) => CollectionId::DatabaseTable(table_name),
            Err(_) => CollectionId::Other(s),
        }
    }
}
