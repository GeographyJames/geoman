use actix_web::{HttpResponse, get};

use crate::ogc::types::common::Collections;

/// The feature collections in the dataset.
#[utoipa::path(
    responses(
        (
            status = 200,
            description = "The feature collections shared by this API. \
            \n\nThe dataset is organized as one or more feature collections. \
            This resource provides information about and access to the \
            collections. \
            \n\nThe response contains the list of collections. For each \
            collection, a link to the items in the collection (path \
            `/collections/{collectionId}/items`, link relation `items`) as \
            well as key information about the collection. This information \
            includes: \
            \n* A local identifier for the collection that is unique for \
            the dataset; \
            \n* A list of coordinate reference systems (CRS) in which \
            geometries may be returned by the server. The first CRS is the \
            default coordinate reference system (the default is always WGS 84 \
            with axis order longitude/latitude); \
            \n* An optional title and description for the collection; \
            \n* An optional extent that can be used to provide an indication \
            of the spatial and temporal extent of the collection - typically \
            derived from the data; \
            \n* An optional indicator about the type of the items in the \
            collection (the default value, if the indicator is not provided, \
            is 'feature').", 
            body = Collections
        )
    )
)]
#[get("")]
#[tracing::instrument]
pub async fn get_collections() -> HttpResponse {
    HttpResponse::Ok().finish()
}
