use actix_web::{
    post,
    web::{self, Json},
};

use domain::{ProjectCollectionId, ProjectCollectionInputDto};

use crate::{
    errors::ApiError, handlers::api::project_collections::CollectionReqPayload,
    postgres::PostgresRepo, types::AuthenticatedUser,
};

#[post("")]
#[tracing::instrument(skip(repo, payload, user))]
pub async fn post_project_collection(
    repo: web::Data<PostgresRepo>,
    payload: Json<CollectionReqPayload>,
    user: web::ReqData<AuthenticatedUser>,
) -> Result<Json<ProjectCollectionId>, ApiError> {
    if payload.project_id.is_none() && !user.admin {
        return Err(ApiError::AdminOnly);
    }
    let collection_input_dto: ProjectCollectionInputDto = payload
        .into_inner()
        .try_into()
        .map_err(ApiError::InvalidCollectionTitle)?;

    if collection_input_dto.project_id.is_some() {
        let title_clashes: bool = sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM app.collections WHERE title = $1 AND project_id IS NULL)",
            collection_input_dto.title.as_ref()
        )
        .fetch_one(&repo.db_pool)
        .await
        .map_err(|e| ApiError::Unexpected(e.into()))?
        .unwrap_or(false);

        if title_clashes {
            return Err(ApiError::DuplicateCollectionName);
        }

        let slug_clashes: bool = sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM app.collections WHERE slug = $1 AND project_id IS NULL)",
            collection_input_dto.slug
        )
        .fetch_one(&repo.db_pool)
        .await
        .map_err(|e| ApiError::Unexpected(e.into()))?
        .unwrap_or(false);

        if slug_clashes {
            return Err(ApiError::DuplicateCollectionSlug);
        }
    }

    let collection_id = repo.insert(&(&collection_input_dto, user.id)).await?;
    Ok(Json(collection_id))
}
