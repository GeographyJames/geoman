use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Default, Debug, PartialEq)]
pub struct TableName(String);

impl Display for TableName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for TableName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl TableName {
    pub fn parse(s: String) -> Result<Self, String> {
        // Basic validation: alphanumeric, underscores, max length
        let first_char = s.chars().next();
        match first_char {
            None => return Err("table name cannot be empty".to_string()),
            Some(first_char) => {
                if first_char.is_numeric() {
                    return Err("table name cannot start with a digit".to_string());
                }
            }
        }
        if s.len() > 63 {
            // PostgreSQL max identifier length
            return Err("table name too long".to_string());
        }
        if !s
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
        {
            return Err(
                "table name can only contain alphanumeric characters and underscores, and hyphens"
                    .to_string(),
            );
        }

        Ok(TableName(s))
    }
}
impl<'de> Deserialize<'de> for TableName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        TableName::parse(s).map_err(serde::de::Error::custom)
    }
}
