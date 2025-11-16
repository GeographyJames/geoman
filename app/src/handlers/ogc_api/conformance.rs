use actix_web::{get, web};
use domain::Project;
use ogc::{ConformanceDeclaration, conformance_classes};
use std::sync::LazyLock;

use crate::{enums::ProjectIdentifier, errors::ApiError, postgres::PostgresRepo};

static CONFORMANCE_DECLARATION: LazyLock<ConformanceDeclaration> = LazyLock::new(|| {
    let mut declaration = ConformanceDeclaration::default();
    declaration.extend(&[conformance_classes::CORE, conformance_classes::GEOJSON]);
    declaration
});

/// API conformance definition
///
/// A list of all conformance classes specified in a standard that
/// the server conforms to.
#[utoipa::path(
    path = "/conformance",
    responses(
        (
            status = 200,
            description = "The URIs of all conformance classes supported by the server.\
                \n\n To support \"generic\" clients that want to access multiple \
                OGC API Features implementations - and not \"just\" a specific \
                API / server, the server declares the conformance classes it \
                implements and conforms to",
            body = ConformanceDeclaration
        ),
    )
)]
#[get("")]
#[tracing::instrument]
pub async fn get_conformance_declaration() -> web::Json<&'static ConformanceDeclaration> {
    web::Json(&CONFORMANCE_DECLARATION)
}

#[get("")]
#[tracing::instrument(skip(repo, project))]
pub async fn get_project_conformance_declaration(
    repo: web::Data<PostgresRepo>,
    project: web::Path<ProjectIdentifier>,
) -> Result<web::Json<&'static ConformanceDeclaration>, ApiError> {
    let _project = repo
        .select_one::<Project>(&project)
        .await?
        .ok_or(ApiError::ProjectNotFound(project.into_inner()))?;
    Ok(web::Json(&CONFORMANCE_DECLARATION))
}
