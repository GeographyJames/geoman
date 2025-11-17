use crate::{
    URLS,
    enums::{self, ProjectIdentifier},
    errors::{ApiError, RepositoryError},
    helpers::get_base_url,
    postgres::{PostgresRepo, project_features::SelectAllParams},
    streaming::ogc_feature_collection_byte_stream,
};
use actix_web::{
    HttpRequest, HttpResponse, get,
    web::{self},
};
use domain::{Collection, IntoOGCFeature, Project, ProjectFeature, ProjectId};
use futures::{Stream, StreamExt, stream};
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
#[tracing::instrument(skip(repo, req, collection, query))]
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
    let mut response_builder = HttpResponse::Ok();
    response_builder.content_type(GEOJSON);
    let response = match collection.as_ref() {
        enums::Collection::Projects => {
            let projects =
                repo.select_all_with_params_streaming::<Project>(query.into_inner().into());
            let bytes = ogc_feature_collection_byte_stream(
                projects,
                collection_url,
                collection.to_string(),
            )
            .await?;
            response_builder.streaming(bytes)
        }
        enums::Collection::Other(_) => {
            let params = SelectAllParams::from_query(query.into_inner(), collection.to_string());
            let features = project_features_stream(collection.to_string(), params, repo).await?;
            let bytes = ogc_feature_collection_byte_stream(
                features,
                collection_url,
                collection.to_string(),
            )
            .await?;
            response_builder.streaming(bytes)
        }
    };
    Ok(response)
}

#[get("/{collection_id}/items")]
#[tracing::instrument(skip(req, repo, path, query))]
pub async fn get_project_features(
    req: HttpRequest,
    repo: web::Data<PostgresRepo>,
    path: web::Path<(ProjectIdentifier, String)>,
    query: web::Query<Query>,
) -> Result<HttpResponse, ApiError> {
    let (project, collection) = path.into_inner();
    let project_row = repo
        .select_one::<Project>(&project)
        .await?
        .ok_or_else(|| ApiError::ProjectNotFound(project.clone()))?;
    let base_url = get_base_url(&req);
    let collection_url = format!(
        "{}{}{}/{}/collections/{}",
        base_url, URLS.ogc_api.base, URLS.ogc_api.project, project, collection
    );
    let mut params = SelectAllParams::from_query(query.into_inner(), collection.clone());
    params.project_id = Some(ProjectId(project_row.id));
    let mut features = project_features_stream(collection.clone(), params, repo).await?;
    // Check if there are any features
    let first_item = features.next().await;
    if first_item.is_none() {
        return Err(ApiError::CollectionNotFound {
            collection_slug: collection,
        });
    }
    let features = stream::iter(first_item.into_iter()).chain(features);

    let bytes = ogc_feature_collection_byte_stream(features, collection_url, collection).await?;
    Ok(HttpResponse::Ok().content_type(GEOJSON).streaming(bytes))
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
    path: web::Path<(enums::Collection, i32)>,
) -> Result<web::Json<ogc::Feature>, ApiError> {
    let (collection, feature_id) = path.into_inner();
    let base_url = get_base_url(&req);
    let collection_url = format!(
        "{}{}/collections/{}",
        base_url, URLS.ogc_api.base, collection
    );
    let feature = match collection {
        enums::Collection::Projects => {
            let identifier = ProjectIdentifier::Id(ProjectId(feature_id));
            repo.select_one::<Project>(&identifier)
                .await?
                .ok_or_else(|| ApiError::ProjectNotFound(identifier))?
                .into_ogc_feature(collection_url)
        }
        enums::Collection::Other(slug) => repo
            .select_one::<ProjectFeature>(feature_id)
            .await?
            .ok_or_else(|| ApiError::FeatureNotFound {
                feature_id,
                collection_slug: slug,
            })?
            .into_ogc_feature(collection_url),
    };

    Ok(web::Json(feature))
}

async fn project_features_stream(
    collection: String,
    params: SelectAllParams,
    repo: web::Data<PostgresRepo>,
) -> Result<impl Stream<Item = Result<ProjectFeature, RepositoryError>>, ApiError> {
    repo.select_one::<Collection>(&collection)
        .await?
        .ok_or_else(|| ApiError::CollectionNotFound {
            collection_slug: collection,
        })?;

    Ok(repo
        .as_ref()
        .select_all_with_params_streaming::<ProjectFeature>(params))
}
