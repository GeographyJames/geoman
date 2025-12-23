use std::fmt::Display;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error(transparent)]
    UnexpectedSqlx(sqlx::Error),
    #[error("no rows returned from database")]
    RowNotFound,
    #[error("unique key violation: {0}")]
    UniqueKeyViolation(UniqueKey),
    #[error("{0}")]
    UnknownUniqueViolation(String),
    #[error("foreign key violation: {0}")]
    ForeignKeyViolation(ForeignKey, sqlx::Error),
    #[error("{0}")]
    UnknownForeignKeyViolation(sqlx::Error),
}

impl From<sqlx::Error> for RepositoryError {
    fn from(error: sqlx::Error) -> Self {
        match error {
            sqlx::Error::Database(ref db_err) if db_err.is_unique_violation() => {
                let key = db_err.constraint().map(|s| UniqueKey(s.to_string()));
                match key {
                    Some(key) => RepositoryError::UniqueKeyViolation(key),
                    None => RepositoryError::UnknownUniqueViolation(error.to_string()),
                }
            }
            sqlx::Error::Database(ref db_err) if db_err.is_foreign_key_violation() => {
                let key = db_err.constraint().map(|s| ForeignKey(s.to_string()));
                match key {
                    Some(key) => RepositoryError::ForeignKeyViolation(key, error),
                    None => RepositoryError::UnknownForeignKeyViolation(error),
                }
            }
            sqlx::Error::RowNotFound => RepositoryError::RowNotFound,
            _ => RepositoryError::UnexpectedSqlx(error),
        }
    }
}
#[derive(Debug, PartialEq)]
pub struct UniqueKey(pub String);

impl Display for UniqueKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl UniqueKey {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, PartialEq)]
pub struct ForeignKey(pub String);

impl Display for ForeignKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ForeignKey {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}
