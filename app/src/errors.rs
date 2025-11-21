use actix_web::{ResponseError, http::StatusCode};

use domain::{ProjectCollectionId, ProjectFeatureId};
use ogcapi_types::common::Crs;
use thiserror::Error;

use crate::enums::ProjectIdentifier;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
    #[error("database error")]
    Database(#[from] RepositoryError),
    #[error("Project '{0}' not found")]
    ProjectNotFound(ProjectIdentifier),
    #[error("Collection '{0}' not found")]
    ProjectCollectionNotFound(ProjectCollectionId),
    #[error("Project feature not found: {0}")]
    ProjectFeatureNotFound(ProjectFeatureId),
    #[error("Unsupported request CRS: {0}")]
    UnsupportedRequestCrs(Crs),
    #[error("Unsupported BBOX CRS: {0}")]
    UnsupportedBboxCrs(Crs),
}

impl ResponseError for ApiError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            ApiError::Unexpected(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::ProjectNotFound(_) => StatusCode::NOT_FOUND,
            ApiError::ProjectCollectionNotFound { .. } => StatusCode::NOT_FOUND,
            ApiError::ProjectFeatureNotFound { .. } => StatusCode::NOT_FOUND,
            ApiError::UnsupportedRequestCrs(_) => StatusCode::BAD_REQUEST,
            ApiError::UnsupportedBboxCrs(_) => StatusCode::BAD_REQUEST,
        }
    }
}

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
}
