use crate::{
    enums::{self, ProjectIdentifier},
    errors::{ApiError, RepositoryError},
    postgres::{
        PostgresRepo,
        project_features::{SelectAllParams, SelectOneParams},
    },
    types::ValidCrs,
};
use actix_web::{
    HttpResponse,
    http::header::{HeaderName, HeaderValue},
    web::{self},
};

use domain::{
    Collection, FeatureIdWithCollectionSlug, IntoOGCFeature, Project, ProjectFeature, ProjectId,
};
use futures::Stream;

pub async fn retrieve_feature_from_database<'a>(
    repo: &PostgresRepo,
    collection: enums::Collection,
    feature_id: i32,
    collection_url: String,
    params: &SelectOneParams<'a>,
) -> Result<ogc::Feature, ApiError> {
    let feature = match collection {
        enums::Collection::Projects => {
            let identifier = ProjectIdentifier::Id(ProjectId(feature_id));
            repo.select_one::<Project>(&identifier)
                .await?
                .ok_or_else(|| ApiError::ProjectNotFound(identifier))?
                .into_ogc_feature(collection_url)
        }
        enums::Collection::Other(collection_slug) => {
            let id = FeatureIdWithCollectionSlug {
                collection_slug,
                id: feature_id,
            };

            repo.select_one_with_params::<ProjectFeature>(&id, params)
                .await?
                .ok_or_else(|| ApiError::FeatureNotFound {
                    feature_id,
                    collection_slug: id.collection_slug,
                })?
                .into_ogc_feature(collection_url)
        }
    };
    Ok(feature)
}

pub fn append_crs_header(response: &mut HttpResponse, crs: &ValidCrs) {
    response.headers_mut().append(
        HeaderName::from_static("content-crs"),
        HeaderValue::from_str(&format!("<{}>", &crs.as_ref().to_string())).unwrap(),
    );
}

pub async fn project_features_stream(
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
