use actix_web::{HttpRequest, get, web};
use anyhow::Context;

use crate::{
    URLS, constants::DB_QUERY_FAIL, errors::ApiError, helpers::get_base_url, repo::PostgresRepo,
};
use ogc::types::common::{Collection, CollectionRow, Collections};

/// The feature collections in the dataset.
#[utoipa::path(
    path = "/collections",
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
#[tracing::instrument(skip(repo, req))]
pub async fn get_collections(
    req: HttpRequest,
    repo: web::Data<PostgresRepo>,
) -> Result<web::Json<Collections>, ApiError> {
    // Build base URL from request

    let base_url = get_base_url(&req);
    let collections_url = format!("{}{}/collections", base_url, URLS.ogc_api.base);

    let collection_rows: Vec<CollectionRow> = repo.select_all().await.context(DB_QUERY_FAIL)?;
    let collections = Collections::from_collection_rows(collection_rows, &collections_url)
        .add_collection(Collection::new(
            "projects".to_string(),
            "Projects".to_string(),
            Some("The projects".to_string()),
            collections_url,
        ));

    Ok(web::Json(collections))
}

/// Get a single collection by ID (slug)
#[utoipa::path(
    get,
    path = "/collections/{collectionId}",
    params(
        ("collectionId" = String, Path, description = "local identifier of a collection")
    ),
    responses(
        (
            status = 200,
            description = "Information about the feature collection with id \
                `collectionId`. \
                \n\nThe response contains a link to the items in the collection \
                (path `/collections/{collectionId}/items`, link relation `items`) \
                as well as key information about the collection. This information \
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
            body = Collection
        ),
        (
            status = 404,
            description = "Collection not found"
        )
    )
)]
#[get("/{collectionId}")]
#[tracing::instrument(skip(repo, req, slug))]
pub async fn get_collection(
    req: HttpRequest,
    slug: web::Path<String>,
    repo: web::Data<PostgresRepo>,
) -> Result<web::Json<Collection>, ApiError> {
    let base_url = get_base_url(&req);
    let collections_url = format!("{}{}/collections", base_url, URLS.ogc_api.base);

    tracing::info!("\n\nhere");

    // Fetch collection from database
    let collection_row = repo
        .select_one::<CollectionRow>(&slug)
        .await
        .context(DB_QUERY_FAIL)?
        .ok_or_else(|| ApiError::NotFound(format!("collection: {}", slug)))?;

    // Map database row to OGC Collection with links
    let collection = Collection::from_collection_row(collection_row, collections_url);

    Ok(web::Json(collection))
}
