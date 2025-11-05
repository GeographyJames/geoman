use actix_web::{HttpResponse, get, web};

use crate::ogc::types::common::ConformanceDeclaration;

/// API conformance definition
///
/// A list of all conformance classes specified in a standard that
/// the server conforms to.
#[utoipa::path(
    get, path = "/conformance", tag = "Capabilities", 
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
pub async fn get_conformance_declaration(
    conformance: web::Data<ConformanceDeclaration>,
) -> HttpResponse {
    HttpResponse::Ok().json(conformance)
}
