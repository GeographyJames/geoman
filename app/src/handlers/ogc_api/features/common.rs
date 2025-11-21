use crate::{
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
    Collection, IntoOGCFeature, Project, ProjectCollectionId, ProjectFeature, ProjectFeatureId,
    ProjectId, enums::CollectionId,
};
use futures::Stream;

pub async fn retrieve_feature_from_database<'a>(
    repo: &PostgresRepo,
    collection: CollectionId,
    feature_id: i32,
    collection_url: String,
    params: &SelectOneParams<'a>,
) -> Result<ogc::Feature, ApiError> {
    let feature = match collection {
        CollectionId::Projects => {
            let project_id = ProjectId(feature_id);
            repo.select_one::<Project>(project_id)
                .await?
                .ok_or_else(|| ApiError::ProjectNotFound(project_id))?
                .into_ogc_feature(collection_url)
        }
        CollectionId::ProjectCollection(collection_id) => {
            let id = ProjectFeatureId {
                collection_id,
                id: feature_id,
            };

            repo.select_one_with_params::<ProjectFeature>(&id, params)
                .await?
                .ok_or_else(|| ApiError::ProjectFeatureNotFound(id))?
                .into_ogc_feature(collection_url)
        }
        CollectionId::Other(_collection) => todo!(),
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
    collection_id: ProjectCollectionId,
    params: SelectAllParams,
    repo: web::Data<PostgresRepo>,
) -> Result<impl Stream<Item = Result<ProjectFeature, RepositoryError>>, ApiError> {
    repo.select_one::<Collection>(collection_id)
        .await?
        .ok_or_else(|| ApiError::ProjectCollectionNotFound(collection_id))?;

    Ok(repo
        .as_ref()
        .select_all_with_params_streaming::<ProjectFeature>(params))
}
