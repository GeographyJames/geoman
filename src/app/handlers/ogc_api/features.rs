use actix_web::{HttpResponse, get, web};

use crate::repo::{PostgresRepo, ogc::CollectionRow};

/// The features in the collection
#[utoipa::path(responses((status = 200, description = "todo!")))]
#[get("/{collection_slug}/items")]
#[tracing::instrument(skip(repo, collection_slug))]
pub async fn get_features(
    repo: web::Data<PostgresRepo>,
    collection_slug: web::Path<String>,
) -> HttpResponse {
    let collection_row: CollectionRow = repo
        .select_by_slug(&collection_slug)
        .await
        .expect("failed to query database")
        .expect("no collection with this slug");
    let collection_id = collection_row.id;
    let features = repo
        .select_features(collection_id)
        .await
        .expect("failed to retrieve feature");
    HttpResponse::Ok().json(features)
}

/// A single feature
#[utoipa::path(
    responses(
        (status = 200, description = "A single feature from the collection"),
        (status = 404, description = "Collection or feature not found")
    )
)]
#[get("/{collection_slug}/items/{feature_id}")]
#[tracing::instrument(skip(repo, path))]
pub async fn get_feature(
    repo: web::Data<PostgresRepo>,
    path: web::Path<(String, i32)>,
) -> HttpResponse {
    let (collection_slug, feature_id) = path.into_inner();

    // Get collection by slug
    let collection_row: CollectionRow = match repo.select_by_slug(&collection_slug).await {
        Ok(Some(row)) => row,
        Ok(None) => return HttpResponse::NotFound().finish(),
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    // Get feature by ID
    match repo.select_feature(collection_row.id, feature_id).await {
        Ok(Some(feature)) => HttpResponse::Ok().json(feature),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
