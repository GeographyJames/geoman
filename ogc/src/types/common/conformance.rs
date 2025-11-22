use serde::Serialize;
use utoipa::ToSchema;

/// The Conformance declaration states the conformance classes from standards or community
/// specifications, identified by a URI, that the API conforms to. Clients can but are not
/// required to use this information. Accessing the Conformance declaration using HTTP GET
/// returns the list of URIs of conformance classes implemented by the server.
#[derive(Serialize, ToSchema)]
pub struct ConformanceDeclaration {
    #[serde(rename = "conformsTo")]
    pub conforms_to: Vec<String>,
}
