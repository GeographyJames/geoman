use crate::{
    handlers::ApiError,
    postgres::PostgresRepo,
    repo::{self},
};
use actix_web::{
    HttpResponse,
    http::header::{HeaderName, HeaderValue},
};

use domain::{
    IntoOGCFeature, Project, ProjectFeature, ProjectFeatureId, ProjectId, enums::CollectionId,
};
use ogcapi_types::common::Crs;

pub async fn retrieve_feature_from_database<'a>(
    repo: &PostgresRepo,
    collection: CollectionId,
    feature_id: i32,
    collection_url: String,
    params: &repo::project_features::SelectOneParams<'a>,
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
        CollectionId::DatabaseTable(_) => todo!(),
    };
    Ok(feature)
}

pub fn append_crs_header(response: &mut HttpResponse, crs: &Crs) {
    response.headers_mut().append(
        HeaderName::from_static("content-crs"),
        HeaderValue::from_str(&format!("<{}>", &crs.to_string())).unwrap(),
    );
}
