use crate::{
    app::URLS,
    ogc::types::features::Query,
    repo::{PostgresRepo, ogc::CollectionRow},
};
use actix_web::{HttpRequest, HttpResponse, get, web};

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

/// Add links to a GeoJSON feature (self and collection)
fn add_feature_links(feature: &mut geojson::Feature, self_href: String, collection_href: String) {
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

    add_links_to_foreign_members(&mut feature.foreign_members, links);
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
#[tracing::instrument(skip(repo, req, collection_slug, query))]
pub async fn get_features(
    req: HttpRequest,
    repo: web::Data<PostgresRepo>,
    collection_slug: web::Path<String>,
    query: web::Query<Query>,
) -> Result<HttpResponse, actix_web::Error> {
    // Build base URL from request
    let base_url = {
        let connection_info = req.connection_info();
        format!("{}://{}", connection_info.scheme(), connection_info.host())
    };

    let collection_row: CollectionRow = match repo
        .select_by_slug(&collection_slug)
        .await
        .expect("failed to query database")
    {
        Some(row) => row,
        None => {
            return Err(actix_web::error::ErrorNotFound(format!(
                "Collection id {} does not exist",
                collection_slug
            )));
        }
    };
    let collection_id = collection_row.id;
    let mut feature_collection = repo
        .select_features(collection_id, query.limit)
        .await
        .expect("failed to retrieve feature");

    // Build URLs
    let collection_url = format!(
        "{}{}/collections/{}",
        base_url, URLS.ogc_api.base, collection_slug
    );
    let items_url = format!("{}/items", collection_url);

    // Add links to the FeatureCollection
    add_collection_links(&mut feature_collection, items_url, collection_url.clone());

    // Add links to each feature
    for feature in feature_collection.features.iter_mut() {
        if let Some(geojson::feature::Id::Number(feature_id)) = &feature.id {
            let feature_url = format!(
                "{}{}/collections/{}/items/{}",
                base_url, URLS.ogc_api.base, collection_slug, feature_id
            );
            add_feature_links(feature, feature_url, collection_url.clone());
        }
    }

    Ok(HttpResponse::Ok().json(feature_collection))
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
) -> HttpResponse {
    let (collection_slug, feature_id) = path.into_inner();

    // Build base URL from request
    let base_url = {
        let connection_info = req.connection_info();
        format!("{}://{}", connection_info.scheme(), connection_info.host())
    };

    // Get collection by slug
    let collection_row: CollectionRow = match repo.select_by_slug(&collection_slug).await {
        Ok(Some(row)) => row,
        Ok(None) => return HttpResponse::NotFound().finish(),
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    // Get feature by ID
    match repo.select_feature(collection_row.id, feature_id).await {
        Ok(Some(mut feature)) => {
            // Add links to the feature
            let feature_url = format!(
                "{}{}/collections/{}/items/{}",
                base_url, URLS.ogc_api.base, collection_slug, feature_id
            );
            let collection_url = format!(
                "{}{}/collections/{}",
                base_url, URLS.ogc_api.base, collection_slug
            );
            add_feature_links(&mut feature, feature_url, collection_url);

            HttpResponse::Ok().json(feature)
        }
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
