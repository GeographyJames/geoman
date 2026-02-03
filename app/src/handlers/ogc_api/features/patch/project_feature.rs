use actix_web::{HttpResponse, patch, web};
use domain::{ProjectCollectionId, ProjectFeatureId, ProjectId, enums::Status};
use serde::{Deserialize, Serialize};

use crate::{AuthenticatedUser, errors::ApiError, postgres::PostgresRepo};

#[derive(Serialize, Default, Deserialize)]
pub struct PatchProjectFeaturePayload {
    pub status: Option<Status>,
    pub primary: Option<bool>,
}

#[patch("/{collectionId}/items/{featureId}")]
#[tracing::instrument(skip(repo, path, user, body))]
pub async fn patch_project_feature(
    path: web::Path<(ProjectId, ProjectCollectionId, i32)>,
    repo: web::Data<PostgresRepo>,
    user: web::ReqData<AuthenticatedUser>,
    body: web::Json<PatchProjectFeaturePayload>,
) -> Result<HttpResponse, ApiError> {
    let (project_id, collection_id, id) = path.into_inner();
    let feature = ProjectFeatureId { collection_id, id };
    repo.update(&(&body.into_inner(), user.id, project_id, feature))
        .await?;
    Ok(HttpResponse::NoContent().finish())
}
