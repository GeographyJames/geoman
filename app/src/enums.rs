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

impl Collection {
    pub fn to_string(&self) -> String {
        match self {
            Collection::Projects => "projects".to_string(),
            Collection::Other(s) => s.clone(),
        }
    }
}

impl std::fmt::Display for Collection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
