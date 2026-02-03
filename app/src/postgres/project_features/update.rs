use domain::{ProjectFeatureId, ProjectId, UserId, enums::Status};
use sqlx::{Acquire, Postgres};

use crate::{
    handlers::ogc_api::features::patch::project_feature::PatchProjectFeaturePayload,
    repo::traits::Update,
};

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
            last_updated = NOW(),
            last_updated_by = $3
        WHERE id= $4
        AND collection_id = $5
        AND project_id = $6
        RETURNING id, collection_id"#,
            &payload.status as &Option<Status>,
            payload.primary,
            user_id.0,
            feature_id.id,
            feature_id.collection_id.0,
            project_id.0
        )
        .fetch_one(&mut *tx)
        .await?;
        tx.commit().await?;
        Ok(ProjectFeatureId {
            collection_id: domain::ProjectCollectionId(result.collection_id),
            id: (result.id),
        })
    }
}
