use crate::{
    app::{
        URLS,
        helpers::{get_base_url, get_collection_row_from_slug},
    },
    constants::DB_QUERY_FAIL,
    domain::{CollectionId, FeatureId},
    ogc::{
        self,
        types::{FeatureCollection, features::Query},
    },
    repo::{
        PostgresRepo,
        ogc::{FeatureRow, features::DbQueryParams},
    },
};
use actix_web::{HttpRequest, get, web};
use sqlx::types::Json;

/// The features in the collection
#[utoipa::path(
    path = "/collections/{collectionId}/items",
    params(
        ("collectionId" = String, Path, description = "local identifier of a collection"),
        Query
    ),
    responses(
        (status = 200, description = "todo!"),
        (status = 404, description = "todo!"))
)]
#[get("/{collectionId}/items")]
#[tracing::instrument(skip(repo, req, slug, query))]
pub async fn get_features(
    req: HttpRequest,
    repo: web::Data<PostgresRepo>,
    slug: web::Path<String>,
    query: web::Query<Query>,
) -> Result<web::Json<ogc::types::FeatureCollection>, actix_web::Error> {
    let base_url = get_base_url(&req);

    let collection_row = get_collection_row_from_slug(&slug, repo.get_ref()).await?;
    let feature_rows = repo
        .select_one_with_params::<Json<Vec<FeatureRow>>>(
            &CollectionId(collection_row.id),
            &DbQueryParams {
                limit: query.limit.map(|l| l as i64),
            },
        )
        .await
        .expect(DB_QUERY_FAIL)
        .ok_or_else(|| {
            actix_web::error::ErrorNotFound(format!(
                "Collection {} does not exist",
                collection_row.slug
            ))
        })?
        .0;
    let collection_url = format!("{}{}/collections/{}", base_url, URLS.ogc_api.base, slug);
    let feature_collection =
        FeatureCollection::from_feature_rows(feature_rows, collection_url, slug.to_string());

    Ok(web::Json(feature_collection))
}

// #[get("/{collectionId}/items")]
// #[tracing::instrument(skip(repo, req, slug, query))]
// pub async fn get_features_streaming(
//     req: HttpRequest,
//     repo: web::Data<PostgresRepo>,
//     slug: web::Path<String>,
//     query: web::Query<Query>,
// ) -> Result<web::Json<FeatureCollection>, actix_web::Error> {
//     let base_url = get_base_url(&req);
//     let collection_row = get_collection_row_from_slug(&slug, repo.get_ref()).await?;
//     let _collection_id = collection_row.id;
//     // Build URLs
//     let collection_url = format!("{}{}/collections/{}", base_url, URLS.ogc_api.base, slug);
//     let _items_url = format!("{}/items", collection_url);

//     let _feature_stream = repo
//         .select_features_streaming(collection_row.id, query.limit)
//         .map(move |res| {
//             res.map(|Json(mut feature)| {
//                 add_feature_links(&mut feature, collection_url.clone());

//                 feature
//             })
//         });

//     todo!()
// }

/// A single feature
#[utoipa::path(
    path = "/collections/{collectionId}/items/{featureId}",
    params(
        ("collectionId" = String, Path, description = "local identifier of a collection"),
        ("featureId" = i32, Path, description = "local identifier of a feature"),
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
) -> Result<web::Json<ogc::types::Feature>, actix_web::Error> {
    let (slug, feature_id) = path.into_inner();
    let base_url = get_base_url(&req);
    let collection_url = format!("{}{}/collections/{}", base_url, URLS.ogc_api.base, slug);
    // Get feature by ID
    let feature_row = repo
        .select_one::<Json<FeatureRow>>(&FeatureId(feature_id))
        .await
        .expect("Failed go retrieve feature from database")
        .ok_or_else(|| {
            actix_web::error::ErrorNotFound(format!("Feature id {} does not exist", feature_id))
        })?
        .0;

    let feature = ogc::types::Feature::from_feature_row(feature_row, collection_url);
    Ok(web::Json(feature))
}
