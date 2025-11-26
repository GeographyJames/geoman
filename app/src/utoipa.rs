use crate::handlers::ogc_api::{
    collections::{__path_get_collection, __path_get_collections},
    conformance::__path_get_conformance_declaration,
    features::{get_feature::__path_get_feature, get_features::__path_get_features},
    landing_page::__path_get_landing_page,
};
use ogc::{Collection, Collections, ConformanceDeclaration, LandingPage};

use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "GeoMan API",
        description = "Standards-compliant OGC API Features with PostGIS integration",
        version = "1.0.0",
    ),
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
)]
pub struct ApiDoc;
