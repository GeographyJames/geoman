use actix_web::{HttpResponse, get, web};

use crate::{app::AppState, ogc::types::common::ConformanceDeclaration};

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
#[tracing::instrument(skip(state))]
pub async fn get_conformance_declaration(state: web::Data<AppState>) -> HttpResponse {
    let conformance = &state.conformance_declaration;
    HttpResponse::Ok().json(conformance)
}
