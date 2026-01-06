use crate::{ProjectCollectionId, TableName};
use serde::{Deserialize, Deserializer, Serialize};
use sqlx::prelude::Type;
use strum::{Display, EnumString};

#[derive(Serialize, Deserialize, Debug, Type, EnumString, Display, PartialEq, Clone, Default)]
#[sqlx(type_name = "app.status", rename_all = "UPPERCASE")]
#[serde(rename_all = "UPPERCASE")]
#[strum(serialize_all = "UPPERCASE")]
pub enum Status {
    #[default]
    Active,
    Archived,
    Deleted,
}

#[derive(Serialize, Deserialize, Type, Clone, Debug, Display, Default)]
#[sqlx(type_name = "app.visibility", rename_all = "UPPERCASE")]
#[serde(rename_all = "UPPERCASE")]
pub enum Visibility {
    Private,
    Team,
    #[default]
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
        s.try_into().map_err(serde::de::Error::custom)
    }
}

impl TryFrom<String> for CollectionId {
    fn try_from(s: String) -> Result<Self, Self::Error> {
        if let Ok(id) = s.parse::<i32>() {
            return Ok(CollectionId::ProjectCollection(ProjectCollectionId(id)));
        }
        if s == "projects" {
            return Ok(CollectionId::Projects);
        }
        match TableName::parse(s.clone()) {
            Ok(table_name) => Ok(CollectionId::DatabaseTable(table_name)),
            _ => Err("invalid collection".to_string()),
        }
    }

    type Error = String;
}
