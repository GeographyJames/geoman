use actix_web::{HttpRequest, get, web};
use domain::{
    Collections, GisDataTable, IntoOGCCollection, Project, ProjectCollection, ProjectCollectionId,
    ProjectId, SupportedCrs, enums::CollectionId,
};

use crate::{
    URLS,
    handlers::ApiError,
    helpers::get_base_url,
    postgres::PostgresRepo,
    repo::project_collections::{SelectAllParams, SelectOneParams},
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
    let base_url = get_base_url(&req);
    let collections_url = format!("{}{}/collections", base_url, URLS.ogc_api.base);

    let gis_data_tables = repo.select_all::<GisDataTable>().await?;

    let collections = Collections {
        project_collections: Vec::new(),
        gis_data_tables,
    };

    let mut ogc_collections = collections.into_ogc_collections(&collections_url);
    ogc_collections
        .collections
        .push(project_collection(&collections_url));
    Ok(web::Json(ogc_collections))
}

#[get("")]
#[tracing::instrument(skip(repo, req, project_id))]
pub async fn get_project_collections(
    req: HttpRequest,
    repo: web::Data<PostgresRepo>,
    project_id: web::Path<ProjectId>,
) -> Result<web::Json<ogcapi_types::common::Collections>, ApiError> {
    let _project_row = repo
        .select_one::<Project>(*project_id)
        .await?
        .ok_or(ApiError::ProjectNotFound(*project_id))?;
    let params = SelectAllParams {
        project_id: *project_id,
    };
    let base_url = get_base_url(&req);
    let collections_url = format!(
        "{}{}{}/{}/collections",
        base_url, URLS.ogc_api.base, URLS.ogc_api.project, project_id
    );

    let (project_collections, _) = repo
        .select_all_with_params::<ProjectCollection>(&params)
        .await?;
    let collections = Collections {
        project_collections,
        gis_data_tables: Vec::new(),
    };

    let ogc_collections = collections.into_ogc_collections(&collections_url);

    Ok(web::Json(ogc_collections))
}

/// Get a single collection by ID
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
            body = ProjectCollection
        ),
        (
            status = 404,
            description = "Collection not found"
        )
    )
)]
#[get("/{collectionId}")]
#[tracing::instrument(skip(repo, req, collection_id))]
pub async fn get_collection(
    req: HttpRequest,
    collection_id: web::Path<CollectionId>,
    repo: web::Data<PostgresRepo>,
) -> Result<web::Json<ogcapi_types::common::Collection>, ApiError> {
    let base_url = get_base_url(&req);
    let collections_url = format!("{}{}/collections", base_url, URLS.ogc_api.base);

    let ogc_collection = match collection_id.into_inner() {
        CollectionId::Projects => project_collection(&collections_url),

        CollectionId::DatabaseTable(table_name) => repo
            .select_one::<GisDataTable>(table_name.clone())
            .await?
            .ok_or_else(|| ApiError::GisDataTableNotFound(table_name))?
            .into_ogc_collection(&collections_url),
        _ => return Err(ApiError::CollectionNotFound),
    };
    Ok(web::Json(ogc_collection))
}

#[get("/{collectionId}")]
#[tracing::instrument(skip(repo, req, path))]
pub async fn get_project_collection(
    req: HttpRequest,
    path: web::Path<(ProjectId, ProjectCollectionId)>,
    repo: web::Data<PostgresRepo>,
) -> Result<web::Json<ogcapi_types::common::Collection>, ApiError> {
    let (project_id, collection_id) = path.into_inner();
    let _project_row = repo
        .select_one::<Project>(project_id)
        .await?
        .ok_or_else(|| ApiError::ProjectNotFound(project_id))?;
    let base_url = get_base_url(&req);
    let collections_url = format!(
        "{}{}{}/{}/collections",
        base_url, URLS.ogc_api.base, URLS.ogc_api.project, project_id
    );

    // Fetch collection from database
    let params = SelectOneParams { project_id };
    let collection = repo
        .select_one_with_params::<ProjectCollection>(collection_id, &params)
        .await?
        .ok_or_else(|| ApiError::ProjectCollectionNotFound(collection_id))?;

    // Map database row to OGC Collection with links
    let ogc_collection = collection.into_ogc_collection(&collections_url);

    Ok(web::Json(ogc_collection))
}

fn project_collection(collections_url: &str) -> ogcapi_types::common::Collection {
    ProjectCollection {
        id: CollectionId::Projects,
        storage_crs: None,
        title: "Projects".to_string(),
        extent: None,
        description: None,
        supported_crs: SupportedCrs::new(None),
    }
    .into_ogc_collection(collections_url)
}
