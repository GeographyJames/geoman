use domain::{ProjectId, UserId, enums::Status, project::ProjectUpdateDto};
use sqlx::{Acquire, Postgres};

use crate::repo::traits::Update;

impl Update for (&ProjectUpdateDto, UserId) {
    type Id = ProjectId;

    async fn update<'a, E>(&self, conn: E) -> Result<Self::Id, crate::repo::RepositoryError>
    where
        Self: Sized,
        E: Acquire<'a, Database = Postgres>,
    {
        let (dto, user_id) = self;
        let mut executor = conn.acquire().await?;

        let result = sqlx::query!(
            r#"
            UPDATE app.projects
            SET
                status = COALESCE($1, status),
                last_updated = NOW(),
                last_updated_by = $2
            WHERE id = $3 AND owner = $2
            RETURNING id
            "#,
            &dto.status as &Option<Status>,
            user_id.0,
            dto.id.0
        )
        .fetch_one(&mut *executor)
        .await?;

        Ok(ProjectId(result.id))
    }
}
