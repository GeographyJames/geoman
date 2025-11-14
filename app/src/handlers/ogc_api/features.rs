use crate::{
    URLS, constants::DB_QUERY_FAIL, enums::Collection, errors::ApiError, helpers::get_base_url,
    streaming::feature_collection_byte_stream,
};
use actix_web::{
    HttpRequest, HttpResponse, get,
    web::{self},
};
use anyhow::Context;
use ogc::types::{
    common::{CollectionRow, conformance_classes::GEOJSON},
    features::{FeatureRow, Query},
};
use postgres::{PostgresRepo, ogc::features::SelectAllParams};

/// The features in the collection
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

// #[tracing::instrument(skip(repo, req, slug, query))]
pub async fn get_features_streaming(
    req: HttpRequest,
    repo: web::Data<PostgresRepo>,
    collection: web::Path<Collection>,
    query: web::Query<Query>,
) -> Result<HttpResponse, ApiError> {
    // Check collection exists
    match collection.as_ref() {
        Collection::Projects => todo!(),
        Collection::Other(_) => {}
    }
    repo.select_one::<CollectionRow>(&collection.to_string())
        .await
        .context(DB_QUERY_FAIL)?
        .ok_or_else(|| ApiError::NotFound(format!("Collection: '{}'", collection)))?;
    let base_url = get_base_url(&req);
    let collection_url = format!(
        "{}{}/collections/{}",
        base_url, URLS.ogc_api.base, &collection
    );
    let params = SelectAllParams::from_query(query.into_inner(), collection.to_string());
    let byte_stream = feature_collection_byte_stream(repo, params, collection_url)?;
    Ok(HttpResponse::Ok()
        .content_type(GEOJSON.to_string())
        .streaming(byte_stream))
}

/// A single feature
#[utoipa::path(
    path = "/collections/{collectionId}/items/{featureId}",
    params(
        ("collectionId" = String, Path, description = "Identifier of a collection"),
        ("featureId" = i32, Path, description = "Identifier of a feature"),
    ),
    responses(
        (status = 200, description = "A single feature from the collection"),
        (status = 404, description = "Collection or feature not found")
    )
)]
#[get("/{collectionId}/items/{featureId}")]
#[tracing::instrument(skip(repo, req, path))]
pub async fn get_feature(
    req: HttpRequest,
    repo: web::Data<PostgresRepo>,
    path: web::Path<(String, i32)>,
) -> Result<web::Json<ogc::types::Feature>, ApiError> {
    let (slug, feature_id) = path.into_inner();
    let base_url = get_base_url(&req);
    let collection_url = format!("{}{}/collections/{}", base_url, URLS.ogc_api.base, slug);
    let feature_row = repo
        .select_one::<FeatureRow>(feature_id)
        .await
        .context(DB_QUERY_FAIL)?
        .ok_or_else(|| {
            ApiError::NotFound(format!(
                "Feature with collection: {}, id: {},",
                slug, feature_id
            ))
        })?;
    let feature = ogc::types::Feature::from_feature_row(feature_row, collection_url);
    Ok(web::Json(feature))
}

// /// The features in the collection
// #[utoipa::path(
//     path = "/collections/{collectionId}/items",
//     params(
//         ("collectionId" = String, Path, description = "Identifier of a collection"),
//         Query
//     ),
//     responses(
//         (status = 200, description = "Features in the collection"),
//         (status = 404, description = "Collection not found"))
// )]
// #[get("/{collectionId}/items")]
// #[tracing::instrument(skip(repo, req, slug, query))]
// pub async fn get_features(
//     req: HttpRequest,
//     repo: web::Data<PostgresRepo>,
//     slug: web::Path<String>,
//     query: web::Query<Query>,
// ) -> Result<web::Json<ogc::types::FeatureCollection>, ApiError> {
//     let base_url = get_base_url(&req);
//     let mut params = SelectAllParams::new(&slug);
//     params.limit = query.limit;

//     let feature_rows = repo
//         .select_all_features_by_collection(&params)
//         .await
//         .context(DB_QUERY_FAIL)?
//         .ok_or_else(|| ApiError::NotFound(format!("Collection: '{}'", slug)))?;
//     let collection_url = format!("{}{}/collections/{}", base_url, URLS.ogc_api.base, slug);
//     let feature_collection =
//         FeatureCollection::from_feature_rows(feature_rows, collection_url, slug.to_string());
//     Ok(web::Json(feature_collection))
// }
