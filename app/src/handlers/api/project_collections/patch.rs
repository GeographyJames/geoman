use actix_web::{HttpResponse, patch, web};
use domain::{CollectionUpdateDto, ProjectCollectionId, enums::Status, name::NameInputDTO};
use serde::{Deserialize, Serialize};

use crate::{errors::ApiError, postgres::PostgresRepo, types::AuthenticatedUser};

fn deserialize_optional_field<'de, T, D>(deserializer: D) -> Result<Option<Option<T>>, D::Error>
where
    T: Deserialize<'de>,
    D: serde::Deserializer<'de>,
{
    Ok(Some(Option::deserialize(deserializer)?))
}

#[derive(Deserialize, Default, Serialize)]
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

    let project_id: Option<i32> = sqlx::query_scalar!(
        "SELECT project_id FROM app.collections WHERE id = $1",
        collection_id.0
    )
    .fetch_optional(&repo.db_pool)
    .await
    .map_err(|e| ApiError::Unexpected(e.into()))?
    .flatten();

    if !user.admin && project_id.is_none() {
        return Err(ApiError::AdminOnly);
    }

    if let Some(ref new_title) = payload.title
        && project_id.is_some()
    {
        let title_clashes: bool = sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM app.collections WHERE title = $1 AND project_id IS NULL)",
            new_title
        )
        .fetch_one(&repo.db_pool)
        .await
        .map_err(|e| ApiError::Unexpected(e.into()))?
        .unwrap_or(false);

        if title_clashes {
            return Err(ApiError::DuplicateCollectionName);
        }

        let new_slug = slug::slugify(new_title);
        let slug_clashes: bool = sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM app.collections WHERE slug = $1 AND project_id IS NULL)",
            new_slug
        )
        .fetch_one(&repo.db_pool)
        .await
        .map_err(|e| ApiError::Unexpected(e.into()))?
        .unwrap_or(false);

        if slug_clashes {
            return Err(ApiError::DuplicateCollectionSlug);
        }
    }

    if let Some(ref status) = payload.status
        && status == &Status::Deleted
    {
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

    let slug = payload.title.as_deref().map(slug::slugify);
    let dto = CollectionUpdateDto {
        id: collection_id,
        title: payload
            .title
            .map(NameInputDTO::parse)
            .transpose()
            .map_err(ApiError::InvalidCollectionTitle)?,
        slug,
        description: payload.description,
        status: payload.status,
    };
    repo.update(&(&dto, user.id)).await?;
    Ok(HttpResponse::NoContent().finish())
}
