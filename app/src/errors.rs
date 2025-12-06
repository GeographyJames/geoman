use actix_web::{ResponseError, http::StatusCode};
use domain::{
    FeatureId, ProjectCollectionId, ProjectFeatureId, ProjectId, TableName, enums::Action,
};

use isocountry::CountryCodeParseErr;
use thiserror::Error;

use crate::{
    constants::db_constraints::{PROJECT_NAME_UNIQUE, PROJECT_SLUG_UNIQUE},
    helpers::error_chain_fmt,
    repo::RepositoryError,
    types::ErrorResponse,
};

#[derive(Error)]
pub enum ApiError {
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
    #[error("Unexpeced database error")]
    UnexpectedDatabase(#[source] RepositoryError),
    #[error("Resource not found")]
    ResourceNotFound(#[source] RepositoryError),
    #[error(transparent)]
    Conflict(RepositoryError),
    #[error("Project '{0}' not found")]
    ProjectNotFound(ProjectId),
    #[error("A project with this name already exists")]
    DuplicateProjectName,
    #[error("The project URL is unavailable")]
    DuplicateProjectSlug,
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
    #[error("Not found")]
    NotFound,
    #[error("User must be a member of a team to {0}")]
    UserWithoutTeam(Action),
}

impl From<RepositoryError> for ApiError {
    fn from(value: RepositoryError) -> Self {
        match value {
            RepositoryError::UnexpectedSqlx(_) => Self::UnexpectedDatabase(value),
            RepositoryError::RowNotFound => ApiError::ResourceNotFound(value),
            RepositoryError::UnknownUniqueViolation(_) => Self::Conflict(value),
            RepositoryError::UniqueKeyViolation(ref unique_key) => match unique_key.as_str() {
                PROJECT_NAME_UNIQUE => ApiError::DuplicateProjectName,
                PROJECT_SLUG_UNIQUE => ApiError::DuplicateProjectSlug,
                _ => ApiError::Conflict(value),
            },
        }
    }
}

impl ResponseError for ApiError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::Unexpected(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::UnexpectedDatabase(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::Conflict(_) => StatusCode::CONFLICT,
            ApiError::ResourceNotFound(_) => StatusCode::NOT_FOUND,
            ApiError::ProjectNotFound(_) => StatusCode::NOT_FOUND,
            ApiError::DuplicateProjectName => StatusCode::CONFLICT,
            ApiError::DuplicateProjectSlug => StatusCode::CONFLICT,
            ApiError::ProjectCollectionNotFound { .. } => StatusCode::NOT_FOUND,
            ApiError::ProjectFeatureNotFound { .. } => StatusCode::NOT_FOUND,
            ApiError::GisDataTableNotFound(_) => StatusCode::NOT_FOUND,
            ApiError::CollectionNotFound => StatusCode::NOT_FOUND,
            ApiError::FeatureNotFound(_) => StatusCode::NOT_FOUND,
            ApiError::ProjectValidation(_) => StatusCode::UNPROCESSABLE_ENTITY,
            ApiError::UserWithoutTeam(_) => StatusCode::FORBIDDEN,
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
