use actix_web::{ResponseError, http::StatusCode};

use domain::{ProjectCollectionId, ProjectFeatureId, ProjectId};
use ogcapi_types::common::Crs;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error)]
pub enum ApiError {
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
    #[error("database error")]
    Database(#[from] RepositoryError),
    #[error("Project '{0}' not found")]
    ProjectNotFound(ProjectId),
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

    fn error_response(&self) -> actix_web::HttpResponse {
        let error_response = ErrorResponse {
            status: self.status_code().as_u16(),
            message: self.to_string(),
            long_message: format!("{:?}", self),
        };
        actix_web::HttpResponse::build(self.status_code()).json(error_response)
    }
}

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub status: u16,
    pub message: String,
    pub long_message: String,
}

impl std::fmt::Debug for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

pub fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    write!(f, "{}", e)?;
    let mut current = e.source();
    while let Some(cause) = current {
        write!(f, "\ncaused by:\n\t{}", cause)?;
        current = cause.source();
    }
    Ok(())
}
