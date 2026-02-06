use actix_web::{HttpResponse, patch, web};
use domain::{CollectionUpdateDto, ProjectCollectionId, enums::Status};
use serde::Deserialize;

use crate::{errors::ApiError, postgres::PostgresRepo, types::AuthenticatedUser};

fn deserialize_optional_field<'de, T, D>(deserializer: D) -> Result<Option<Option<T>>, D::Error>
where
    T: Deserialize<'de>,
    D: serde::Deserializer<'de>,
{
    Ok(Some(Option::deserialize(deserializer)?))
}

#[derive(Deserialize)]
pub struct PatchCollectionPayload {
    pub title: Option<String>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub description: Option<Option<String>>,
    pub status: Option<Status>,
}

#[patch("/{id}")]
#[tracing::instrument(skip(repo, body, user))]
pub async fn patch_collection(
    id: web::Path<ProjectCollectionId>,
    body: web::Json<PatchCollectionPayload>,
    repo: web::Data<PostgresRepo>,
    user: web::ReqData<AuthenticatedUser>,
) -> Result<HttpResponse, ApiError> {
    let mut payload = body.into_inner();
    let collection_id = id.into_inner();

    // Check if user owns this collection or is an admin
    if !user.admin {
        let added_by: i32 = sqlx::query_scalar!(
            r#"
            SELECT added_by
            FROM app.collections
            WHERE id = $1
            "#,
            collection_id.0
        )
        .fetch_one(&repo.db_pool)
        .await
        .map_err(|e| ApiError::Unexpected(e.into()))?;

        if added_by != user.id.0 {
            return Err(ApiError::NotCollectionOwner);
        }
    }

    if let Some(ref status) = payload.status {
        if status == &Status::Deleted {
            // Check if collection has active or archived features
            let feature_count: i64 = sqlx::query_scalar!(
                r#"
                SELECT COUNT(*) as "count!"
                FROM app.project_features
                WHERE collection_id = $1
                AND status IN ('ACTIVE', 'ARCHIVED')
                "#,
                collection_id.0
            )
            .fetch_one(&repo.db_pool)
            .await
            .map_err(|e| ApiError::Unexpected(e.into()))?;

            if feature_count > 0 {
                return Err(ApiError::CollectionHasFeatures);
            }

            payload.title = Some(uuid::Uuid::new_v4().to_string())
        }
    }

    let dto = CollectionUpdateDto {
        id: collection_id,
        title: payload.title,
        description: payload.description,
        status: payload.status,
    };
    repo.update(&(&dto, user.id)).await?;
    Ok(HttpResponse::NoContent().finish())
}
