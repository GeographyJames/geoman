use domain::{ProjectId, Slug};
use serde::{Deserialize, Deserializer};
use strum::EnumString;

/// Available runtime environments
#[derive(Deserialize, EnumString, strum::Display, Clone)]
#[strum(ascii_case_insensitive, serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum GeoManEnvironment {
    Development,
    Demo,
    Staging,
    Production,
    CiPipeline,
}

#[derive(Clone, Debug)]
pub enum Collection {
    Projects,
    Other(String),
}

#[derive(Clone, Debug)]
pub enum ProjectIdentifier {
    Id(ProjectId),
    Slug(String),
}

impl TryInto<Slug> for Collection {
    type Error = String;

    fn try_into(self) -> Result<Slug, Self::Error> {
        Slug::parse(self.to_string())
    }
}

impl<'de> Deserialize<'de> for ProjectIdentifier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let identifier = match s.parse::<i32>() {
            Ok(id) => Self::Id(ProjectId(id)),
            Err(_) => Self::Slug(s),
        };
        Ok(identifier)
    }
}

impl std::fmt::Display for ProjectIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProjectIdentifier::Id(id) => write!(f, "{}", id.0),
            ProjectIdentifier::Slug(slug) => write!(f, "{}", slug),
        }
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

impl std::fmt::Display for Collection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Collection::Projects => "projects".to_string(),
            Collection::Other(s) => s.clone(),
        };
        write!(f, "{}", s)
    }
}
