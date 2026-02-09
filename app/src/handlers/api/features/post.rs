use actix_web::{HttpResponse, post};

#[tracing::instrument()]
#[post("{projectId}/{collectionId}")]
pub async fn post_project_feature_shapefile() -> HttpResponse {
    HttpResponse::Ok().finish()
}
