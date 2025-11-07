use crate::app::handlers::ogc_api::{
    collections::{__path_get_collection, __path_get_collections},
    conformance::__path_get_conformance_declaration,
    features::{__path_get_feature, __path_get_features},
    landing_page::__path_get_landing_page,
};
use crate::ogc::types::common::{Collection, Collections, ConformanceDeclaration, LandingPage};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        get_landing_page,
        get_conformance_declaration,
        get_collections,
        get_collection,
        get_features,
        get_feature,
    ),
    components(
        schemas(LandingPage, ConformanceDeclaration, Collections, Collection)
    ),
    tags(
        (name = "OGC API", description = "OGC API Features endpoints")
    ),
    servers(
        (url = "/ogcapi", description = "OGC API Features server")
    )
)]
pub struct ApiDoc;
