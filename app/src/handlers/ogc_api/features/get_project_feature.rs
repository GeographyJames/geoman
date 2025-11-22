use crate::{
    URLS,
    handlers::{
        ApiError,
        ogc_api::features::{
            Query,
            common::{append_crs_header, retrieve_feature_from_database},
        },
    },
    helpers::get_base_url,
    postgres::PostgresRepo,
    repo::project_features::SelectOneParams,
    types::ValidCrs,
};
use actix_web::{
    HttpRequest, HttpResponse, get,
    web::{self},
};

use domain::{Project, ProjectId, enums::CollectionId};
use ogcapi_types::common::Crs;

#[get("/{collectionId}/items/{featureId}")]
#[tracing::instrument(skip(repo, req, path, query))]
pub async fn get_project_feature(
    req: HttpRequest,
    repo: web::Data<PostgresRepo>,
    path: web::Path<(ProjectId, CollectionId, i32)>,
    query: web::Query<Query>,
) -> Result<HttpResponse, ApiError> {
    let (project_id, collection_id, feature_id) = path.into_inner();
    let _project_row = repo
        .select_one::<Project>(project_id)
        .await?
        .ok_or_else(|| ApiError::ProjectNotFound(project_id))?;
    let valid_crs: Vec<Crs> = repo.select_all().await?;
    let Query { crs, .. } = query.into_inner();
    let request_crs = ValidCrs::new(&valid_crs, crs).map_err(ApiError::UnsupportedRequestCrs)?;

    let base_url = get_base_url(&req);
    let collection_url = format!(
        "{}{}{}/{}/collections/{}",
        base_url, URLS.ogc_api.base, URLS.ogc_api.project, project_id, collection_id
    );
    let params = SelectOneParams {
        project_id: Some(project_id),
        crs: &request_crs,
    };
    let feature =
        retrieve_feature_from_database(&repo, collection_id, feature_id, collection_url, &params)
            .await?;
    let mut response = HttpResponse::Ok().json(feature);
    append_crs_header(&mut response, &request_crs);
    Ok(response)
}
