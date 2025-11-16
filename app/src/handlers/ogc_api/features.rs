use crate::{
    URLS, enums,
    errors::{ApiError, RepositoryError},
    helpers::get_base_url,
    postgres::{PostgresRepo, project_features::SelectAllParams},
    streaming::ogc_feature_collection_byte_stream,
};
use actix_web::{
    HttpRequest, HttpResponse, get,
    web::{self},
};
use domain::{Collection, IntoOGCFeature, Project, ProjectFeature};
use futures::Stream;
use ogc::{conformance_classes::GEOJSON, features::Query};

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
    collection: web::Path<enums::Collection>,
    query: web::Query<Query>,
) -> Result<HttpResponse, ApiError> {
    let base_url = get_base_url(&req);
    let collection_url = format!(
        "{}{}/collections/{}",
        base_url, URLS.ogc_api.base, &collection
    );
    let response = match collection.as_ref() {
        enums::Collection::Projects => {
            let projects = repo.select_all_streaming::<Project>();
            let bytes = ogc_feature_collection_byte_stream(
                projects,
                collection_url,
                collection.to_string(),
            )?;
            HttpResponse::Ok().content_type(GEOJSON).streaming(bytes)
        }
        enums::Collection::Other(_) => {
            let features =
                project_features_stream(collection.to_string(), query.into_inner(), repo).await?;
            let bytes = ogc_feature_collection_byte_stream(
                features,
                collection_url,
                collection.to_string(),
            )?;
            HttpResponse::Ok().content_type(GEOJSON).streaming(bytes)
        }
    };
    Ok(response)
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
) -> Result<web::Json<ogc::Feature>, ApiError> {
    let (slug, feature_id) = path.into_inner();
    let base_url = get_base_url(&req);
    let collection_url = format!("{}{}/collections/{}", base_url, URLS.ogc_api.base, slug);
    let feature_row = repo
        .select_one::<ProjectFeature>(feature_id)
        .await?
        .ok_or_else(|| ApiError::FeatureNotFound {
            feature_id,
            collection_slug: slug,
        })?;
    let feature = feature_row.into_ogc_feature(collection_url);
    Ok(web::Json(feature))
}

async fn project_features_stream(
    collection: String,
    query: Query,
    repo: web::Data<PostgresRepo>,
) -> Result<impl Stream<Item = Result<ProjectFeature, RepositoryError>>, ApiError> {
    repo.select_one::<Collection>(&collection)
        .await?
        .ok_or_else(|| ApiError::CollectionNotFound {
            collection_slug: collection.clone(),
        })?;
    let params = SelectAllParams::from_query(query, collection);
    Ok(repo
        .as_ref()
        .select_all_with_params_streaming::<ProjectFeature>(params))
}
