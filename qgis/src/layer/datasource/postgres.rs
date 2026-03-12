use serde::{Deserialize, Serialize};

use crate::layer::WkbType;

#[derive(Clone)]
pub struct PgConfig {
    pub db_name: String,
    pub port: u16,
    pub host: String,
    pub sslmode: SslMode,
}

#[allow(non_snake_case)]
#[derive(Clone)]
pub struct PgDataSource {
    pub pg_config: PgConfig,
    pub key: String,
    pub srid: Option<u16>,
    pub r#type: Option<WkbType>,
    pub checkPrimaryKeyUnicity: u8,
    pub source: PgSource,
    pub geometry_col: String,
    pub authcfg: Option<String>,
}

impl std::fmt::Display for PgDataSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut parts = Vec::new();
        parts.push(format!("dbname='{}'", self.pg_config.db_name));
        parts.push(format!("host={}", self.pg_config.host));
        parts.push(format!("port={}", self.pg_config.port));
        parts.push(format!("sslmode={}", self.pg_config.sslmode));

        if let Some(ref cfg) = self.authcfg {
            parts.push(format!("authcfg={}", cfg));
        }

        parts.push(format!("key='{}'", self.key));

        if let Some(srid) = self.srid {
            parts.push(format!("srid={}", srid));
        }

        if let Some(ref t) = self.r#type {
            parts.push(format!("type={}", t));
        }
        parts.push(format!(
            "checkPrimaryKeyUnicity='{}'",
            self.checkPrimaryKeyUnicity
        ));

        // Handle different source types
        match &self.source {
            PgSource::SQL(sql) => {
                parts.push(format!(r#"table="({})" ({})"#, sql, self.geometry_col));
            }
            PgSource::PgTable(pg_table) => {
                parts.push(format!("table={} ({})", pg_table, self.geometry_col));
            }
        };

        write!(f, "{}", parts.join(" "))
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum PgSource {
    SQL(String),
    PgTable(PgTable),
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct PgTable {
    pub schema: String,
    pub table_name: String,
}

impl std::fmt::Display for PgTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, r#""{}"."{}""#, self.schema, self.table_name)
    }
}

impl Serialize for PgDataSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[derive(Clone, Debug)]
pub enum SslMode {
    Require,
    Disable,
}

impl std::fmt::Display for SslMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Require => write!(f, "require"),
            Self::Disable => write!(f, "disable"),
        }
    }
}

impl From<bool> for SslMode {
    fn from(value: bool) -> Self {
        match value {
            true => Self::Require,
            false => Self::Disable,
        }
    }
}

impl Serialize for SslMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
