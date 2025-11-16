use actix_web::{get, web};
use ogc::{ConformanceDeclaration, conformance_classes};
use std::sync::LazyLock;

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
