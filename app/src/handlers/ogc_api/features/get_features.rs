/// The features in the collection
use crate::{
    URLS,
    enums::{self},
    errors::ApiError,
    handlers::ogc_api::features::{
        Query,
        common::{append_crs_header, project_features_stream},
    },
    helpers::get_base_url,
    postgres::{PostgresRepo, project_features::SelectAllParams, projects},
    streaming::ogc_feature_collection_byte_stream,
    types::ValidCrs,
};
use actix_web::{
    HttpRequest, HttpResponse, get,
    web::{self},
};

use domain::Project;
use ogcapi_types::common::{Crs, media_type::GEO_JSON};

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
    let valid_crs: Vec<Crs> = repo.select_all().await?;

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

    let base_url = get_base_url(&req);

    let collection_url = format!(
        "{}{}/collections/{}",
        base_url, URLS.ogc_api.base, &collection
    );
    let mut response_builder = HttpResponse::Ok();
    response_builder.content_type(GEO_JSON);
    let mut response = match collection.as_ref() {
        enums::Collection::Projects => {
            let params = projects::SelectAllParams { limit };
            let projects = repo.select_all_with_params_streaming::<Project>(params);
            let bytes = ogc_feature_collection_byte_stream(
                projects,
                collection_url,
                collection.to_string(),
            )
            .await?;
            response_builder.streaming(bytes)
        }
        enums::Collection::Other(_) => {
            let params = SelectAllParams {
                limit,
                slug: collection.clone(),
                project_id: None,
                crs: request_crs.clone(),
                bbox: bbox,
                bbox_crs: bbox_crs,
            };
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
    append_crs_header(&mut response, &request_crs);
    Ok(response)
}
