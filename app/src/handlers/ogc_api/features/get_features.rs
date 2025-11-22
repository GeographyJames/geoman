/// The features in the collection
use crate::{
    URLS,
    handlers::{
        ApiError,
        ogc_api::features::{
            Query,
            common::{append_crs_header, project_features_stream},
        },
    },
    helpers::get_base_url,
    postgres::PostgresRepo,
    repo::{project, project_features},
    streaming::ogc_feature_collection_byte_stream,
    types::ValidCrs,
};
use actix_web::{
    HttpRequest, HttpResponse, get,
    web::{self},
};

use anyhow::Context;
use domain::{IntoOGCFeature, Project, enums::CollectionId};
use ogc::{Feature, FeatureCollection};
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
#[tracing::instrument(skip(repo, req, collection_id, query))]
pub async fn get_features(
    req: HttpRequest,
    repo: web::Data<PostgresRepo>,
    collection_id: web::Path<CollectionId>,
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
        base_url, URLS.ogc_api.base, collection_id
    );
    let mut response_builder = HttpResponse::Ok();
    response_builder.content_type(GEO_JSON);
    let mut response = match *collection_id {
        CollectionId::Projects => {
            let params = project::SelectAllParams { limit };
            let (projects, number_matched) = repo.select_all_with_params::<Project>(params).await?;
            let features: Vec<Feature> = projects
                .into_iter()
                .map(|p| p.into_ogc_feature(collection_url.clone()))
                .collect();
            let collection = FeatureCollection::new(
                &collection_url,
                collection_id.to_string(),
                features,
                number_matched.0,
            );
            HttpResponse::Ok().json(collection)
        }
        CollectionId::ProjectCollection(project_collection_id) => {
            let params = project_features::SelectAllParams {
                limit,
                collection_id: project_collection_id,
                project_id: None,
                crs: request_crs.clone(),
                bbox,
                bbox_crs,
            };
            let features = project_features_stream(project_collection_id, params, repo).await?;
            let bytes = ogc_feature_collection_byte_stream(
                features,
                collection_url,
                collection_id.into_inner(),
            )
            .await
            .context("failed to create byte stream")?;
            response_builder.streaming(bytes)
        }
        CollectionId::Other(_) => todo!(),
    };
    append_crs_header(&mut response, &request_crs);
    Ok(response)
}
