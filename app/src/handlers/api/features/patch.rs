use actix_web::{HttpResponse, patch, web};
use domain::{FeatureId, LayoutId, ProjectCollectionId, ProjectFeatureId, ProjectId, enums::Status};
use serde::{Deserialize, Serialize};

use crate::{
    AuthenticatedUser, constants::TURBINE_LAYOUTS_COLLECTION_ID, errors::ApiError,
    postgres::PostgresRepo,
};

#[derive(Serialize, Default, Deserialize)]
pub struct PatchProjectFeaturePayload {
    pub status: Option<Status>,
    pub primary: Option<bool>,
    pub name: Option<String>,
}

#[patch("{projectId}/{collectionId}/{featureId}")]
#[tracing::instrument(skip(repo, path, user, body))]
pub async fn patch_project_feature(
    path: web::Path<(ProjectId, ProjectCollectionId, FeatureId)>,
    repo: web::Data<PostgresRepo>,
    user: web::ReqData<AuthenticatedUser>,
    body: web::Json<PatchProjectFeaturePayload>,
) -> Result<HttpResponse, ApiError> {
    let (project_id, collection_id, feature_id) = path.into_inner();
    let payload = body.into_inner();

    if collection_id.0 == TURBINE_LAYOUTS_COLLECTION_ID {
        repo.update(&(&payload, user.id, project_id, LayoutId(feature_id.0)))
            .await?;
        return Ok(HttpResponse::NoContent().finish());
    }

    let feature = ProjectFeatureId {
        collection_id,
        feature_id,
    };

    repo.update(&(&payload, user.id, project_id, feature))
        .await?;
    Ok(HttpResponse::NoContent().finish())
}
