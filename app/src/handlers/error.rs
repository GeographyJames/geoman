use actix_web::{ResponseError, http::StatusCode};
use domain::{FeatureId, ProjectCollectionId, ProjectFeatureId, ProjectId, TableName};

use thiserror::Error;

use crate::{helpers::error_chain_fmt, repo::RepositoryError, types::ErrorResponse};

#[derive(Error)]
pub enum ApiError {
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
    #[error("database error")]
    Database(#[from] RepositoryError),
    #[error("Project '{0}' not found")]
    ProjectNotFound(ProjectId),
    #[error("Project collection '{0}' not found")]
    ProjectCollectionNotFound(ProjectCollectionId),
    #[error("Project feature not found: {0}")]
    ProjectFeatureNotFound(ProjectFeatureId),
    #[error("GIS data table not found: {0}")]
    GisDataTableNotFound(TableName),
    #[error("Collection not found")]
    CollectionNotFound,
    #[error("Feature not found: {0}")]
    FeatureNotFound(FeatureId),
    #[error("Authentication failed")]
    Authentication(#[source] anyhow::Error),
}

impl ResponseError for ApiError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            ApiError::Unexpected(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::Database(RepositoryError::Sqlx(sqlx::Error::RowNotFound)) => {
                StatusCode::NOT_FOUND
            }
            ApiError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::ProjectNotFound(_) => StatusCode::NOT_FOUND,
            ApiError::ProjectCollectionNotFound { .. } => StatusCode::NOT_FOUND,
            ApiError::ProjectFeatureNotFound { .. } => StatusCode::NOT_FOUND,
            ApiError::GisDataTableNotFound(_) => StatusCode::NOT_FOUND,
            ApiError::CollectionNotFound => StatusCode::NOT_FOUND,
            ApiError::FeatureNotFound(_) => StatusCode::NOT_FOUND,
            ApiError::Authentication(_) => StatusCode::UNAUTHORIZED,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        let error_response = ErrorResponse {
            message: self.to_string(),
            long_message: format!("{:?}", self),
        };
        actix_web::HttpResponse::build(self.status_code()).json(error_response)
    }
}

impl std::fmt::Debug for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}
