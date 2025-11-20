use crate::{
    URLS,
    enums::ProjectIdentifier,
    errors::ApiError,
    handlers::ogc_api::features::{
        Query,
        common::{append_crs_header, retrieve_feature_from_database},
    },
    helpers::get_base_url,
    postgres::{PostgresRepo, project_features::SelectOneParams},
    types::ValidCrs,
};
use actix_web::{
    HttpRequest, HttpResponse, get,
    web::{self},
};

use domain::{Project, ProjectId};
use ogcapi_types::common::Crs;

#[get("/{collectionId}/items/{featureId}")]
#[tracing::instrument(skip(repo, req, path, query))]
pub async fn get_project_feature(
    req: HttpRequest,
    repo: web::Data<PostgresRepo>,
    path: web::Path<(ProjectIdentifier, domain::enums::Collection, i32)>,
    query: web::Query<Query>,
) -> Result<HttpResponse, ApiError> {
    let valid_crs: Vec<Crs> = repo.select_all().await?;
    let (project, collection, feature_id) = path.into_inner();
    let Query { crs, .. } = query.into_inner();
    let request_crs = ValidCrs::new(&valid_crs, crs).map_err(ApiError::UnsupportedRequestCrs)?;
    let project_row = repo
        .select_one::<Project>(&project)
        .await?
        .ok_or_else(|| ApiError::ProjectNotFound(project.clone()))?;
    let base_url = get_base_url(&req);
    let collection_url = format!(
        "{}{}{}/{}/collections/{}",
        base_url, URLS.ogc_api.base, URLS.ogc_api.project, project, collection
    );
    let params = SelectOneParams {
        project_id: Some(ProjectId(project_row.id)),
        crs: &request_crs,
    };
    let feature =
        retrieve_feature_from_database(&repo, collection, feature_id, collection_url, &params)
            .await?;
    let mut response = HttpResponse::Ok().json(feature);
    append_crs_header(&mut response, &request_crs);
    Ok(response)
}
