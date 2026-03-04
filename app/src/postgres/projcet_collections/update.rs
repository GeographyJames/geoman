use domain::{CollectionUpdateDto, ProjectCollectionId, UserId, enums::Status};
use sqlx::{Acquire, Postgres};

use crate::repo::traits::Update;

impl Update for (&CollectionUpdateDto, UserId) {
    type Id = ProjectCollectionId;

    async fn update<'a, A>(&self, conn: A) -> Result<Self::Id, crate::repo::RepositoryError>
    where
        Self: Sized,
        A: Acquire<'a, Database = Postgres>,
    {
        let (dto, user_id) = self;

        let mut executor = conn.acquire().await?;
        sqlx::query_scalar!(
            r#"
            UPDATE app.collections
            SET
                title = COALESCE($1, title),
                slug = COALESCE($2, slug),
                description = CASE WHEN $3 THEN $4 ELSE description END,
                status = COALESCE($5, status),
                last_updated_by = $6
            WHERE id = $7
            RETURNING id AS "id: ProjectCollectionId"
            "#,
            dto.title.as_ref().map(|t| t.as_ref()),
            dto.slug,
            dto.description.is_some(),
            dto.description.clone().flatten(),
            &dto.status as &Option<Status>,
            user_id.0,
            dto.id.0
        )
        .fetch_one(&mut *executor)
        .await
        .map_err(Into::into)
    }
}
