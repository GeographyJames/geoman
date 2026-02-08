use domain::{ProjectFeatureId, ProjectId, UserId, enums::Status};
use sqlx::{Acquire, Postgres};

use crate::{handlers::api::features::patch::PatchProjectFeaturePayload, repo::traits::Update};

impl Update
    for (
        &PatchProjectFeaturePayload,
        UserId,
        ProjectId,
        ProjectFeatureId,
    )
{
    type Id = ProjectFeatureId;

    async fn update<'a, E>(&self, conn: E) -> Result<Self::Id, crate::repo::RepositoryError>
    where
        E: Acquire<'a, Database = Postgres>,
    {
        let (payload, user_id, project_id, feature_id) = self;
        let mut tx = conn.begin().await?;
        if payload.primary == Some(true) {
            sqlx::query!(
                r#"
            UPDATE app.project_features
            SET is_primary = false,
                last_updated = NOW(),
                last_updated_by = $1
            WHERE collection_id = $2
            AND project_id = $3
            AND is_primary = true"#,
                user_id.0,
                feature_id.collection_id.0,
                project_id.0
            )
            .execute(&mut *tx)
            .await?;
        }
        let result = sqlx::query!(
            r#"
        UPDATE app.project_features
        SET status = COALESCE($1, status),
            is_primary = COALESCE($2, is_primary),
            name = COALESCE($3, name),
            last_updated = NOW(),
            last_updated_by = $4
        WHERE id= $5
        AND collection_id = $6
        AND project_id = $7
        RETURNING id, collection_id"#,
            &payload.status as &Option<Status>,
            payload.primary,
            &payload.name as &Option<String>,
            user_id.0,
            feature_id.feature_id.0,
            feature_id.collection_id.0,
            project_id.0
        )
        .fetch_one(&mut *tx)
        .await?;
        tx.commit().await?;
        Ok(ProjectFeatureId {
            collection_id: domain::ProjectCollectionId(result.collection_id),
            feature_id: domain::FeatureId(result.id),
        })
    }
}
