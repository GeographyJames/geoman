use crate::{
    URLS,
    handlers::{
        ApiError,
        ogc_api::features::{Query, common::append_crs_header},
    },
    helpers::get_base_url,
    postgres::PostgresRepo,
    repo::{project_collections, project_features::SelectAllParams},
    streaming::ogc_feature_collection_byte_stream,
};
use actix_web::{
    HttpRequest, HttpResponse, get,
    web::{self},
};

use domain::{Project, ProjectCollection, ProjectCollectionId, ProjectFeature, ProjectId};

use ogcapi_types::common::media_type::GEO_JSON;

#[get("/{collectionId}/items")]
#[tracing::instrument(skip(req, repo, path, query))]
pub async fn get_project_features(
    req: HttpRequest,
    repo: web::Data<PostgresRepo>,
    path: web::Path<(ProjectId, ProjectCollectionId)>,
    query: web::Query<Query>,
) -> Result<HttpResponse, ApiError> {
    let (project_id, collection_id) = path.into_inner();
    let _project_row = repo
        .select_one::<Project>(project_id)
        .await?
        .ok_or_else(|| ApiError::ProjectNotFound(project_id))?;

    repo.select_one_with_params::<ProjectCollection>(
        collection_id,
        &project_collections::SelectOneParams { project_id },
    )
    .await?
    .ok_or_else(|| ApiError::ProjectCollectionNotFound(collection_id))?;

    let Query {
        limit,
        bbox,
        bbox_crs,
        crs,
        ..
    } = query.into_inner();

    let base_url = get_base_url(&req);
    let collection_url = format!(
        "{}{}{}/{}/collections/{}",
        base_url, URLS.ogc_api.base, URLS.ogc_api.project, project_id, collection_id
    );

    let params = SelectAllParams {
        limit,
        collection_id,
        project_id,
        crs: crs.clone(),
        bbox,
        bbox_crs,
    };

    let features = repo.select_all_with_params_streaming::<ProjectFeature>(params);

    let bytes =
        ogc_feature_collection_byte_stream(features, collection_url, collection_id.into()).await?;
    let mut response = HttpResponse::Ok().content_type(GEO_JSON).streaming(bytes);
    append_crs_header(&mut response, &crs);

    Ok(response)
}
