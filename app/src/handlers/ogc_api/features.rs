use crate::{
    URLS,
    constants::DB_QUERY_FAIL,
    enums::Collection,
    errors::ApiError,
    helpers::get_base_url,
    repo::{PostgresRepo, project_features::SelectAllParams},
    streaming::feature_collection_byte_stream,
};
use actix_web::{
    Either, HttpRequest, HttpResponse, get,
    web::{self, Json},
};
use anyhow::Context;
use domain::{IntoOGCFeature, Project, ProjectFeature};
use ogc::types::{
    FeatureCollection,
    common::{CollectionRow, conformance_classes::GEOJSON},
    features::Query,
};

/// The features in the collection
#[utoipa::path(
    path = "/collections/{collectionId}/items",
    params(
        ("collectionId" = String, Path, description = "Identifier of a collection"),
        Query
    ),
    responses(
        (status = 200, description = "Features in the collection"),
        (status = 404, description = "Collection not found"))
)]
#[get("/{collectionId}/items")]

// #[tracing::instrument(skip(repo, req, slug, query))]
pub async fn get_features(
    req: HttpRequest,
    repo: web::Data<PostgresRepo>,
    collection: web::Path<Collection>,
    query: web::Query<Query>,
) -> Result<Either<Json<FeatureCollection>, HttpResponse>, ApiError> {
    let base_url = get_base_url(&req);
    let collection_url = format!(
        "{}{}/collections/{}",
        base_url, URLS.ogc_api.base, &collection
    );
    match collection.as_ref() {
        Collection::Projects => Ok(Either::Left(
            get_projects(&repo, collection_url, Collection::Projects.to_string()).await?,
        )),
        Collection::Other(_) => Ok(Either::Right(
            get_project_features_streaming(
                collection.to_string(),
                collection_url,
                query.into_inner(),
                repo,
            )
            .await?,
        )),
    }
}

/// A single feature
#[utoipa::path(
    path = "/collections/{collectionId}/items/{featureId}",
    params(
        ("collectionId" = String, Path, description = "Identifier of a collection"),
        ("featureId" = i32, Path, description = "Identifier of a feature"),
    ),
    responses(
        (status = 200, description = "A single feature from the collection"),
        (status = 404, description = "Collection or feature not found")
    )
)]
#[get("/{collectionId}/items/{featureId}")]
#[tracing::instrument(skip(repo, req, path))]
pub async fn get_feature(
    req: HttpRequest,
    repo: web::Data<PostgresRepo>,
    path: web::Path<(String, i32)>,
) -> Result<web::Json<ogc::types::Feature>, ApiError> {
    let (slug, feature_id) = path.into_inner();
    let base_url = get_base_url(&req);
    let collection_url = format!("{}{}/collections/{}", base_url, URLS.ogc_api.base, slug);
    let feature_row = repo
        .select_one::<ProjectFeature>(feature_id)
        .await
        .context(DB_QUERY_FAIL)?
        .ok_or_else(|| {
            ApiError::NotFound(format!(
                "Feature with collection: {}, id: {},",
                slug, feature_id
            ))
        })?;
    let feature = feature_row.into_ogc_feature(collection_url);
    Ok(web::Json(feature))
}

async fn get_project_features_streaming(
    collection: String,
    collection_url: String,
    query: Query,
    repo: web::Data<PostgresRepo>,
) -> Result<HttpResponse, ApiError> {
    repo.select_one::<CollectionRow>(&collection)
        .await
        .context(DB_QUERY_FAIL)?
        .ok_or_else(|| ApiError::NotFound(format!("Collection: '{}'", collection)))?;
    let params = SelectAllParams::from_query(query, collection);
    let byte_stream = feature_collection_byte_stream(repo, params, collection_url)?;

    Ok(HttpResponse::Ok()
        .content_type(GEOJSON.to_string())
        .streaming(byte_stream))
}

async fn get_projects(
    repo: &PostgresRepo,
    collection_url: String,
    slug: String,
) -> Result<Json<FeatureCollection>, ApiError> {
    let project_rows: Vec<Project> = repo.select_all().await.context(DB_QUERY_FAIL)?;
    let projects = project_rows
        .into_iter()
        .map(|p| p.into_ogc_feature(collection_url.clone()))
        .collect();
    let projects_collection =
        FeatureCollection::new(collection_url, slug).append_features(projects);
    Ok(Json(projects_collection))
}
