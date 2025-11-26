/// The features in the collection
use crate::{
    URLS,
    constants::GIS_DATA_SCHEMA,
    handlers::{
        ApiError,
        ogc_api::features::{Query, common::append_crs_header},
    },
    helpers::get_base_url,
    postgres::PostgresRepo,
    repo::{features, project},
    streaming::ogc_feature_collection_byte_stream,
};
use actix_web::{
    HttpRequest, HttpResponse, get,
    web::{self},
};

use domain::{GisDataTable, IntoOGCFeature, Project, enums::CollectionId};
use ogcapi_types::common::media_type::GEO_JSON;

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
#[tracing::instrument(skip(repo, req, path, query))]
pub async fn get_features(
    req: HttpRequest,
    repo: web::Data<PostgresRepo>,
    path: web::Path<CollectionId>,
    query: web::Query<Query>,
) -> Result<HttpResponse, ApiError> {
    let collection_id = path.into_inner();

    let Query {
        limit,
        bbox,

        bbox_crs,
        crs,
        ..
    } = query.into_inner();

    let base_url = get_base_url(&req);

    let collection_url = format!(
        "{}{}/collections/{}",
        base_url, URLS.ogc_api.base, collection_id
    );
    let mut response_builder = HttpResponse::Ok();
    response_builder.content_type(GEO_JSON);
    let mut response = match collection_id {
        CollectionId::Projects => {
            let params = project::SelectAllParams {
                limit,
                crs: &crs,
                _bbox: bbox,
                _bbox_crs: bbox_crs.as_ref(),
            };
            let (projects, number_matched) = repo.select_all_with_params::<Project>(params).await?;
            let features: Vec<ogc::Feature> = projects
                .into_iter()
                .map(|p| p.into_ogc_feature(collection_url.clone()))
                .collect();
            let collection = ogc::FeatureCollection::new(
                &collection_url,
                collection_id.to_string(),
                features,
                number_matched.0,
            );
            HttpResponse::Ok().json(collection)
        }
        CollectionId::DatabaseTable(table) => {
            let _table_row = repo
                .select_one::<GisDataTable>(table.clone())
                .await?
                .ok_or(ApiError::CollectionNotFound)?;

            let params = features::SelectAllParams {
                schema: GIS_DATA_SCHEMA,
                table: table.clone(),
                limit,
                bbox,
                bbox_crs,
                crs: crs.clone(),
            };
            let features = repo.select_all_with_params_streaming::<domain::Feature>(params);
            let bytes = ogc_feature_collection_byte_stream(
                features,
                collection_url,
                CollectionId::DatabaseTable(table),
            )
            .await?;
            HttpResponse::Ok().content_type(GEO_JSON).streaming(bytes)
        }
        _ => return Err(ApiError::CollectionNotFound),
    };
    append_crs_header(&mut response, &crs);
    Ok(response)
}
