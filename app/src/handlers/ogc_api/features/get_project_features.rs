use crate::{
    URLS,
    enums::{self, ProjectIdentifier},
    errors::ApiError,
    handlers::ogc_api::features::{
        Query,
        common::{append_crs_header, project_features_stream},
    },
    helpers::get_base_url,
    postgres::{PostgresRepo, project_features::SelectAllParams},
    streaming::ogc_feature_collection_byte_stream,
    types::ValidCrs,
};
use actix_web::{
    HttpRequest, HttpResponse, get,
    web::{self},
};

use domain::{Project, ProjectId};

use ogcapi_types::common::{Crs, media_type::GEO_JSON};

#[get("/{collectionId}/items")]
#[tracing::instrument(skip(req, repo, path, query))]
pub async fn get_project_features(
    req: HttpRequest,
    repo: web::Data<PostgresRepo>,
    path: web::Path<(ProjectIdentifier, enums::Collection)>,
    query: web::Query<Query>,
) -> Result<HttpResponse, ApiError> {
    let valid_crs: Vec<Crs> = repo.select_all().await?;
    let (project, collection) = path.into_inner();
    let Query {
        limit,
        bbox,
        bbox_crs,
        crs,
        ..
    } = query.into_inner();
    let request_crs = ValidCrs::new(&valid_crs, crs).map_err(ApiError::UnsupportedRequestCrs)?;
    let bbox_crs = bbox_crs
        .map(|c| ValidCrs::new(&valid_crs, c).map_err(ApiError::UnsupportedBboxCrs))
        .transpose()?;
    let project_row = repo
        .select_one::<Project>(&project)
        .await?
        .ok_or_else(|| ApiError::ProjectNotFound(project.clone()))?;
    let base_url = get_base_url(&req);
    let collection_url = format!(
        "{}{}{}/{}/collections/{}",
        base_url, URLS.ogc_api.base, URLS.ogc_api.project, project, collection
    );

    let mut params = SelectAllParams {
        limit,
        slug: collection.clone(),
        project_id: Some(ProjectId(project_row.id)),
        crs: request_crs.clone(),
        bbox,
        bbox_crs,
    };
    params.project_id = Some(ProjectId(project_row.id));
    let features = project_features_stream(collection.to_string(), params, repo).await?;

    let bytes =
        ogc_feature_collection_byte_stream(features, collection_url, collection.to_string())
            .await?;
    let mut response = HttpResponse::Ok().content_type(GEO_JSON).streaming(bytes);
    append_crs_header(&mut response, &request_crs);

    Ok(response)
}
