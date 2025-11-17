use actix_web::{HttpRequest, get, web};
use domain::{Collection, Project, ProjectId};

use crate::{
    URLS,
    enums::{self, ProjectIdentifier},
    errors::ApiError,
    helpers::get_base_url,
    postgres::{PostgresRepo, collections::SelectAllParams},
};

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
) -> Result<web::Json<ogc::Collections>, ApiError> {
    // Build base URL from request

    let base_url = get_base_url(&req);
    let collections_url = format!("{}{}/collections", base_url, URLS.ogc_api.base);

    let collections: Vec<Collection> = repo.select_all().await?;
    let ogc_collections = collections
        .into_iter()
        .map(|c| c.into_ogc_collection(&collections_url))
        .collect();

    let ogc_collections = ogc::Collections::new(&collections_url)
        .append_collections(ogc_collections)
        .add_collection(ogc::Collection::new(
            enums::Collection::Projects.to_string(),
            "Projects".to_string(),
            Some("The projects".to_string()),
            &collections_url,
        ));

    Ok(web::Json(ogc_collections))
}

#[get("")]
#[tracing::instrument(skip(repo, req, project))]
pub async fn get_project_collections(
    req: HttpRequest,
    repo: web::Data<PostgresRepo>,
    project: web::Path<ProjectIdentifier>,
) -> Result<web::Json<ogc::Collections>, ApiError> {
    let project_row = repo
        .select_one::<Project>(&project)
        .await?
        .ok_or(ApiError::ProjectNotFound(project.clone()))?;
    let params = SelectAllParams {
        project_id: ProjectId(project_row.id),
    };
    let base_url = get_base_url(&req);
    let collections_url = format!(
        "{}{}/{}/collections",
        base_url, URLS.ogc_api.project, project
    );
    let collections: Vec<Collection> = repo.select_all_with_params(params).await?;
    let ogc_collections = collections
        .into_iter()
        .map(|c| c.into_ogc_collection(&collections_url))
        .collect();
    let ogc_collections =
        ogc::Collections::new(&collections_url).append_collections(ogc_collections);
    Ok(web::Json(ogc_collections))
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
) -> Result<web::Json<ogc::Collection>, ApiError> {
    let base_url = get_base_url(&req);
    let collections_url = format!("{}{}/collections", base_url, URLS.ogc_api.base);

    // Fetch collection from database
    let collection = repo.select_one::<Collection>(&slug).await?.ok_or_else(|| {
        ApiError::CollectionNotFound {
            collection_slug: slug.into_inner(),
        }
    })?;

    // Map database row to OGC Collection with links
    let ogc_collection = collection.into_ogc_collection(&collections_url);

    Ok(web::Json(ogc_collection))
}
