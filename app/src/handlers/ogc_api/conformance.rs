use actix_web::{get, web};
use domain::{ProjectId, project::ProjectName};
use ogcapi_types::common::Conformance;
use std::sync::LazyLock;

use crate::{handlers::ApiError, postgres::PostgresRepo};

static CONFORMANCE_DECLARATION: LazyLock<Conformance> = LazyLock::new(|| {
    let mut declaration = Conformance::default();
    declaration.extend(&[
        "http://www.opengis.net/spec/ogcapi-features-1/1.0/conf/core",
        "http://www.opengis.net/spec/ogcapi-features-1/1.0/conf/geojson",
        "http://www.opengis.net/spec/ogcapi-features-2/1.0/conf/crs",
        "http://www.opengis.net/spec/ogcapi-features-1/1.0/req/oas30",
    ]);
    declaration
});

/// API conformance definition
///
/// A list of all conformance classes specified in a standard that
/// the server conforms to.
#[utoipa::path(
    path = "/conformance",
    tag = "OGC API",
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
pub async fn get_conformance_declaration() -> web::Json<&'static Conformance> {
    web::Json(&CONFORMANCE_DECLARATION)
}

#[get("")]
#[tracing::instrument(skip(repo, project_id))]
pub async fn get_project_conformance_declaration(
    repo: web::Data<PostgresRepo>,
    project_id: web::Path<ProjectId>,
) -> Result<web::Json<&'static Conformance>, ApiError> {
    let _project_row = repo
        .select_one::<ProjectName>(*project_id)
        .await?
        .ok_or(ApiError::ProjectNotFound(*project_id))?;
    Ok(web::Json(&CONFORMANCE_DECLARATION))
}
