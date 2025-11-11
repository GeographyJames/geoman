use crate::{
    app::{
        URLS,
        constants::DB_QUERY_FAIL,
        helpers::{get_base_url, get_collection_row_from_slug},
    },
    ogc::types::features::Query,
    repo::PostgresRepo,
};
use actix_web::{HttpRequest, get, web};

use futures::StreamExt;
use geojson::{Feature, FeatureCollection};
use sqlx::types::Json;

/// Helper to add links to foreign_members
fn add_links_to_foreign_members(
    foreign_members: &mut Option<serde_json::Map<String, serde_json::Value>>,
    links: serde_json::Value,
) {
    if let Some(members) = foreign_members.as_mut() {
        members.insert("links".to_string(), links);
    } else {
        let mut members = serde_json::Map::new();
        members.insert("links".to_string(), links);
        *foreign_members = Some(members);
    }
}

/// Add links to a GeoJSON FeatureCollection (self and collection)
fn add_collection_links(
    feature_collection: &mut geojson::FeatureCollection,
    self_href: String,
    collection_href: String,
) {
    let links = serde_json::json!([
        {
            "href": self_href,
            "rel": "self",
            "type": "application/geo+json"
        },
        {
            "href": collection_href,
            "rel": "collection",
            "type": "application/json"
        }
    ]);

    add_links_to_foreign_members(&mut feature_collection.foreign_members, links);
}

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
) -> Result<web::Json<FeatureCollection>, actix_web::Error> {
    let base_url = get_base_url(&req);

    let collection_row = get_collection_row_from_slug(&slug, repo.get_ref()).await?;
    let mut feature_collection = repo
        .select_features(collection_row.id, query.limit)
        .await
        .expect(DB_QUERY_FAIL);

    // Build URLs
    let collection_url = format!("{}{}/collections/{}", base_url, URLS.ogc_api.base, slug);
    let items_url = format!("{}/items", collection_url);

    // Add links to the FeatureCollection
    add_collection_links(&mut feature_collection, items_url, collection_url.clone());

    // Add links to each feature
    for feature in feature_collection.features.iter_mut() {
        add_feature_links(feature, collection_url.clone());
    }

    Ok(web::Json(feature_collection))
}

#[get("/{collectionId}/items")]
#[tracing::instrument(skip(repo, req, slug, query))]
pub async fn get_features_streaming(
    req: HttpRequest,
    repo: web::Data<PostgresRepo>,
    slug: web::Path<String>,
    query: web::Query<Query>,
) -> Result<web::Json<FeatureCollection>, actix_web::Error> {
    let base_url = get_base_url(&req);
    let collection_row = get_collection_row_from_slug(&slug, repo.get_ref()).await?;
    let _collection_id = collection_row.id;
    // Build URLs
    let collection_url = format!("{}{}/collections/{}", base_url, URLS.ogc_api.base, slug);
    let _items_url = format!("{}/items", collection_url);

    let _feature_stream = repo
        .select_features_streaming(collection_row.id, query.limit)
        .map(move |res| {
            res.map(|Json(mut feature)| {
                add_feature_links(&mut feature, collection_url.clone());

                feature
            })
        });

    todo!()
}

/// Add links to a GeoJSON feature (self and collection)
fn add_feature_links(feature: &mut geojson::Feature, collection_url: String) {
    let feature_id = match feature.id.as_ref().expect("Feature has no id") {
        geojson::feature::Id::String(_) => panic!("feature id is a string (should be number"),
        geojson::feature::Id::Number(n) => n.as_i64().expect("feature id is not valid i64"),
    };
    let links = serde_json::json!([
        {
            "href": format!("{collection_url}/items/{feature_id}", ),
            "rel": "self",
            "type": "application/geo+json"
        },
        {
            "href": collection_url,
            "rel": "collection",
            "type": "application/json"
        }
    ]);

    add_links_to_foreign_members(&mut feature.foreign_members, links);
}

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
) -> Result<web::Json<Feature>, actix_web::Error> {
    let (slug, feature_id) = path.into_inner();
    let base_url = get_base_url(&req);
    let collection_row = get_collection_row_from_slug(&slug, repo.get_ref()).await?;
    let collection_url = format!("{}{}/collections/{}", base_url, URLS.ogc_api.base, slug);
    // Get feature by ID
    match repo
        .select_feature(collection_row.id, feature_id)
        .await
        .expect("Failed go retrieve feature from database")
    {
        Some(mut feature) => {
            // Add links to the feature
            add_feature_links(&mut feature, collection_url);
            Ok(web::Json(feature))
        }
        None => Err(actix_web::error::ErrorNotFound(format!(
            "Feature id {} does not exist",
            feature_id
        ))),
    }
}
