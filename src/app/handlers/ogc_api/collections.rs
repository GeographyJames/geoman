use actix_web::{HttpRequest, HttpResponse, get, web};

use crate::{
    app::URLS,
    ogc::types::common::link_relations::{DATA, SELF},
    ogc::types::common::media_types::JSON,
    ogc::types::common::{Collection, Collections, Link},
    repo::{PostgresRepo, ogc::CollectionRow},
};

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
#[tracing::instrument(skip(repo, req))]
pub async fn get_collections(req: HttpRequest, repo: web::Data<PostgresRepo>) -> HttpResponse {
    // Build base URL from request

    let base_url = {
        let connection_info = req.connection_info();
        format!("{}://{}", connection_info.scheme(), connection_info.host())
    };
    let collections_url = format!("{}{}/collections", base_url, URLS.ogc_api.base);

    // Fetch collections from database
    let collection_rows: Vec<CollectionRow> = repo
        .select_all()
        .await
        .expect("failed to retrieve collections from database");

    // Map database rows to OGC Collections with links
    let collections: Vec<Collection> = collection_rows
        .into_iter()
        .map(
            |CollectionRow {
                 slug,
                 title,
                 description,
                 ..
             }| {
                Collection {
                    id: slug.clone(),
                    title,
                    description,
                    links: vec![
                        Link::new(format!("{}/{}", collections_url, slug), SELF).mediatype(JSON),
                        Link::new(format!("{}/{}/items", collections_url, slug), DATA)
                            .mediatype(JSON)
                            .title("Items"),
                    ],
                }
            },
        )
        .collect();

    // Build response with top-level links
    let response = Collections {
        links: vec![Link::new(&collections_url, SELF).mediatype(JSON)],
        collections,
    };

    HttpResponse::Ok().json(response)
}

/// Get a single collection by ID (slug)
#[utoipa::path(
    get,
    path = "/{collectionId}",
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
#[get("/{collection_id}")]
#[tracing::instrument(skip(repo, req, collection_id))]
pub async fn get_collection(
    req: HttpRequest,
    collection_id: web::Path<String>,
    repo: web::Data<PostgresRepo>,
) -> HttpResponse {
    // Build base URL from request
    let base_url = {
        let connection_info = req.connection_info();
        format!("{}://{}", connection_info.scheme(), connection_info.host())
    };
    let collections_url = format!("{}{}/collections", base_url, URLS.ogc_api.base);

    // Fetch collection from database
    let collection_row: CollectionRow = match repo.select_by_slug(&collection_id).await {
        Ok(Some(row)) => row,
        Ok(None) => return HttpResponse::NotFound().finish(),
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    // Map database row to OGC Collection with links
    let collection = Collection {
        id: collection_row.slug.clone(),
        title: collection_row.title,
        description: collection_row.description,
        links: vec![
            Link::new(format!("{}/{}", collections_url, collection_row.slug), SELF).mediatype(JSON),
            Link::new(
                format!("{}/{}/items", collections_url, collection_row.slug),
                DATA,
            )
            .mediatype(JSON)
            .title("Items"),
        ],
    };

    HttpResponse::Ok().json(collection)
}
