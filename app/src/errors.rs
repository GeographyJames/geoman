use actix_web::{ResponseError, http::StatusCode};
use domain::{
    FeatureId, ProjectCollectionId, ProjectFeatureId, ProjectId, TableName,
    project::{ProjectName, ProjectNameInputDTO},
};

use isocountry::CountryCodeParseErr;
use thiserror::Error;

use crate::{helpers::error_chain_fmt, repo::RepositoryError, types::ErrorResponse};

#[derive(Error)]
pub enum ApiError {
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
    #[error("Database error: {0}")]
    Database(#[from] RepositoryError),
    #[error("Project '{0}' not found")]
    ProjectNotFound(ProjectId),
    #[error("A project with the name '{0}' already exists")]
    DuplicateProjectName(ProjectNameInputDTO),
    #[error("Project name '{0}' is too similar to existing project name: '{1}'")]
    DuplicateProjectSlug(ProjectNameInputDTO, ProjectName),
    #[error("Project collection '{0}' not found")]
    ProjectCollectionNotFound(ProjectCollectionId),
    #[error("Project feature '{0}' not found")]
    ProjectFeatureNotFound(ProjectFeatureId),
    #[error("GIS data table '{0}' not found")]
    GisDataTableNotFound(TableName),
    #[error("Collection not found")]
    CollectionNotFound,
    #[error("Feature '{0}' not found")]
    FeatureNotFound(FeatureId),
    #[error("Failed to validate project: {0}")]
    ProjectValidation(#[from] ProjectValidationError),
}

impl ResponseError for ApiError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            ApiError::Unexpected(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::Database(RepositoryError::Sqlx(sqlx::Error::RowNotFound)) => {
                StatusCode::NOT_FOUND
            }
            ApiError::Database(RepositoryError::Sqlx(sqlx::Error::Database(err)))
                if err.is_unique_violation() =>
            {
                StatusCode::CONFLICT
            }

            ApiError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::ProjectNotFound(_) => StatusCode::NOT_FOUND,
            ApiError::DuplicateProjectName(_) => StatusCode::CONFLICT,
            ApiError::DuplicateProjectSlug(_, _) => StatusCode::CONFLICT,
            ApiError::ProjectCollectionNotFound { .. } => StatusCode::NOT_FOUND,
            ApiError::ProjectFeatureNotFound { .. } => StatusCode::NOT_FOUND,
            ApiError::GisDataTableNotFound(_) => StatusCode::NOT_FOUND,
            ApiError::CollectionNotFound => StatusCode::NOT_FOUND,
            ApiError::FeatureNotFound(_) => StatusCode::NOT_FOUND,
            ApiError::ProjectValidation(_) => StatusCode::UNPROCESSABLE_ENTITY,
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
        match self {
            // For database errors, just show the top-level message to avoid repetition
            ApiError::Database(_) => write!(f, "{}", self),
            // For other errors, show the full chain
            _ => error_chain_fmt(self, f),
        }
    }
}

#[derive(Error)]
pub enum ProjectValidationError {
    #[error("invalid country code: {0}")]
    InvalidCountryCode(#[from] CountryCodeParseErr),
    #[error("invalid project name: {0}")]
    InvalidProjectName(String),
}

impl std::fmt::Debug for ProjectValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}
