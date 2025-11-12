use crate::{
    app::{URLS, errors::ApiError, helpers::get_base_url},
    constants::DB_QUERY_FAIL,
    domain::FeatureId,
    ogc::{
        self,
        types::{common::media_types::GEOJSON, features::Query},
    },
    repo::{
        PostgresRepo,
        models::ogc::{CollectionRow, FeatureRow},
        postgres::features::SelectAllParams,
    },
};
use actix_web::{
    HttpRequest, HttpResponse, get,
    web::{self, Bytes},
};
use anyhow::Context;
use futures::StreamExt;

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
#[tracing::instrument(skip(repo, req, slug, query))]
// #[tracing::instrument(skip(repo, req, slug, query))]
pub async fn get_features_streaming(
    req: HttpRequest,
    repo: web::Data<PostgresRepo>,
    slug: web::Path<String>,
    query: web::Query<Query>,
) -> Result<HttpResponse, ApiError> {
    // Check collection exists
    repo.select_one::<CollectionRow>(slug.as_str())
        .await
        .context(DB_QUERY_FAIL)?
        .ok_or_else(|| ApiError::NotFound(format!("Collection: '{}'", slug)))?;
    let base_url = get_base_url(&req);
    let collection_url = format!("{}{}/collections/{}", base_url, URLS.ogc_api.base, &slug);
    let slug_clone = slug.clone();
    let url_clone = collection_url.clone();

    let opening_stream = futures::stream::once(async move {
        Ok::<_, sqlx::Error>(Bytes::from(opening_json(&slug_clone, &url_clone)))
    });

    let mut params = SelectAllParams::new(&slug);
    params.limit = query.limit;

    let mut first = true;
    let feature_stream = repo
        .select_all_with_params_streaming(params)
        .map(move |res| {
            res.and_then(|feature_row| {
                let feature =
                    ogc::types::Feature::from_feature_row(feature_row, collection_url.clone());
                let mut bytes = if first {
                    first = false;
                    Vec::new()
                } else {
                    vec![b',']
                };
                serde_json::to_writer(&mut bytes, &feature)
                    .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
                Ok(Bytes::from(bytes))
            })
        });

    let closing_stream =
        futures::stream::once(async move { Ok::<_, sqlx::Error>(Bytes::from("]}")) });
    let byte_stream = opening_stream.chain(feature_stream).chain(closing_stream);

    Ok(HttpResponse::Ok()
        .content_type(GEOJSON.to_string())
        .streaming(byte_stream))
}

fn opening_json(slug: &str, collection_url: &str) -> String {
    format!(
        r#"{{"type":"FeatureCollection","id":"{}","links":{},"features":["#,
        slug,
        serde_json::to_string(&[ogc::types::common::Link::new(
            format!("{}/items", collection_url),
            ogc::types::common::link_relations::SELF
        )
        .mediatype(ogc::types::common::media_types::MediaType::GeoJson)])
        .unwrap()
    )
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
        .select_one::<FeatureRow>(&FeatureId(feature_id))
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
