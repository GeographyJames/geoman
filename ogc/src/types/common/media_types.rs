use crate::constants::OPEN_API_JSON;
use serde::{Deserialize, Deserializer, Serialize};

/// Media Type for `application/json`
pub const JSON: &str = "application/json";

/// Media Type for `application/geo+json`
pub const GEOJSON: &str = "application/geo+json";

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MediaType {
    Json,
    GeoJson,
    OpenApi,
    Other(String),
}

impl MediaType {
    pub fn as_str(&self) -> &str {
        match self {
            MediaType::Json => JSON,
            MediaType::GeoJson => GEOJSON,
            MediaType::OpenApi => OPEN_API_JSON,
            MediaType::Other(s) => s,
        }
    }
}

impl Serialize for MediaType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for MediaType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.to_lowercase().as_str() {
            JSON => Ok(MediaType::Json),
            GEOJSON => Ok(MediaType::GeoJson),
            OPEN_API_JSON => Ok(MediaType::OpenApi),
            _ => Ok(MediaType::Other(s)), // Catch-all for unknown types
        }
    }
}
