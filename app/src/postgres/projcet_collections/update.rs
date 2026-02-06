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

        let description_provided = dto.description.is_some();
        let description_value = dto.description.as_ref().and_then(|d| d.as_deref());

        let mut executor = conn.acquire().await?;
        sqlx::query_scalar!(
            r#"
            UPDATE app.collections
            SET
                title = COALESCE($1, title),
                description = CASE WHEN $2 THEN $3 ELSE description END,
                status = COALESCE($4, status),
                last_updated_by = $5
            WHERE id = $6
            RETURNING id AS "id: ProjectCollectionId"
            "#,
            dto.title,
            description_provided,
            description_value,
            &dto.status as &Option<Status>,
            user_id.0,
            dto.id.0
        )
        .fetch_one(&mut *executor)
        .await
        .map_err(Into::into)
    }
}
