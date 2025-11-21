use actix_web::{HttpRequest, get, web};
use domain::{
    Collection, Collections, Project, ProjectCollectionId, ProjectId, enums::CollectionId,
};
use ogcapi_types::common::Crs;

use crate::{
    URLS,
    enums::ProjectIdentifier,
    errors::ApiError,
    helpers::get_base_url,
    postgres::{
        PostgresRepo,
        collections::{SelectAllParams, SelectOneParams},
    },
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
) -> Result<web::Json<ogcapi_types::common::Collections>, ApiError> {
    // Build base URL from request

    let base_url = get_base_url(&req);
    let collections_url = format!("{}{}/collections", base_url, URLS.ogc_api.base);
    let supported_crs = repo.select_all::<Crs>().await?;

    let collections: Collections = repo.select_all::<Collection>().await?.into();
    let mut ogc_collections =
        collections.into_ogc_collections(&collections_url, supported_crs.clone());
    ogc_collections
        .collections
        .push(project_collection(&collections_url, supported_crs));
    Ok(web::Json(ogc_collections))
}

#[get("")]
#[tracing::instrument(skip(repo, req, project))]
pub async fn get_project_collections(
    req: HttpRequest,
    repo: web::Data<PostgresRepo>,
    project: web::Path<ProjectIdentifier>,
) -> Result<web::Json<ogcapi_types::common::Collections>, ApiError> {
    let project_row = repo
        .select_one::<Project>(&project)
        .await?
        .ok_or(ApiError::ProjectNotFound(project.clone()))?;
    let params = SelectAllParams {
        project_id: ProjectId(project_row.id),
    };
    let base_url = get_base_url(&req);
    let collections_url = format!(
        "{}{}{}/{}/collections",
        base_url, URLS.ogc_api.base, URLS.ogc_api.project, project
    );
    let supported_crs = repo.select_all::<Crs>().await?;
    let collections: Collections = repo
        .select_all_with_params::<Collection>(&params)
        .await?
        .into();
    let ogc_collections = collections.into_ogc_collections(&collections_url, supported_crs);

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
#[tracing::instrument(skip(repo, req, collection))]
pub async fn get_collection(
    req: HttpRequest,
    collection: web::Path<CollectionId>,
    repo: web::Data<PostgresRepo>,
) -> Result<web::Json<ogcapi_types::common::Collection>, ApiError> {
    let base_url = get_base_url(&req);
    let collections_url = format!("{}{}/collections", base_url, URLS.ogc_api.base);
    let supported_crs = repo.select_all::<Crs>().await?;

    let ogc_collection = match collection.into_inner() {
        CollectionId::Projects => project_collection(&collections_url, supported_crs.clone()),
        CollectionId::ProjectCollection(id) => repo
            .select_one::<Collection>(id)
            .await?
            .ok_or_else(|| ApiError::ProjectCollectionNotFound(id))?
            .into_ogc_collection(&collections_url, supported_crs),

        CollectionId::Other(_) => todo!(),
    };
    Ok(web::Json(ogc_collection))
}

#[get("/{collectionId}")]
#[tracing::instrument(skip(repo, req, path))]
pub async fn get_project_collection(
    req: HttpRequest,
    path: web::Path<(ProjectIdentifier, ProjectCollectionId)>,
    repo: web::Data<PostgresRepo>,
) -> Result<web::Json<ogcapi_types::common::Collection>, ApiError> {
    let (project, collection_id) = path.into_inner();
    let project_row = repo
        .select_one::<Project>(&project)
        .await?
        .ok_or_else(|| ApiError::ProjectNotFound(project.clone()))?;
    let base_url = get_base_url(&req);
    let collections_url = format!(
        "{}{}{}/{}/collections",
        base_url, URLS.ogc_api.base, URLS.ogc_api.project, project
    );
    let supported_crs = repo.select_all::<Crs>().await?;
    // Fetch collection from database
    let params = SelectOneParams {
        project_id: ProjectId(project_row.id),
    };
    let collection = repo
        .select_one_with_params::<Collection>(collection_id, &params)
        .await?
        .ok_or_else(|| ApiError::ProjectCollectionNotFound(collection_id))?;

    // Map database row to OGC Collection with links
    let ogc_collection = collection.into_ogc_collection(&collections_url, supported_crs);

    Ok(web::Json(ogc_collection))
}

fn project_collection(collections_url: &str, crs: Vec<Crs>) -> ogcapi_types::common::Collection {
    Collection {
        id: CollectionId::Projects,
        storage_crs_srid: None,
        title: "Projects".to_string(),
        extent: None,
        description: None,
    }
    .into_ogc_collection(collections_url, crs)
}
