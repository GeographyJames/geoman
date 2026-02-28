use domain::{LayoutId, ProjectId, UserId, enums::Status};
use sqlx::{Acquire, Postgres};

use crate::{handlers::api::features::patch::PatchProjectFeaturePayload, repo::traits::Update};

impl Update for (&PatchProjectFeaturePayload, UserId, ProjectId, LayoutId) {
    type Id = LayoutId;

    async fn update<'a, E>(&self, conn: E) -> Result<Self::Id, crate::repo::RepositoryError>
    where
        E: Acquire<'a, Database = Postgres>,
    {
        let (payload, user_id, project_id, layout_id) = self;
        let mut tx = conn.begin().await?;

        if payload.primary == Some(true) {
            sqlx::query!(
                r#"
                UPDATE app.turbine_layouts
                SET is_primary = false,
                    last_updated = NOW(),
                    last_updated_by = $1
                WHERE project_id = $2
                  AND is_primary = true"#,
                user_id.0,
                project_id.0
            )
            .execute(&mut *tx)
            .await?;
        }

        let result = sqlx::query!(
            r#"
            UPDATE app.turbine_layouts
            SET status = COALESCE($1, status),
                is_primary = COALESCE($2, is_primary),
                name = COALESCE($3, name),
                last_updated = NOW(),
                last_updated_by = $4
            WHERE id = $5
              AND project_id = $6
            RETURNING id"#,
            &payload.status as &Option<Status>,
            payload.primary,
            &payload.name as &Option<String>,
            user_id.0,
            layout_id.0,
            project_id.0
        )
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(LayoutId(result.id))
    }
}
