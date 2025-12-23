use domain::{ApiKeyInputDTO, KeyId, UserId};
use sqlx::{Acquire, Postgres};

use crate::repo::traits::Insert;

impl Insert for (&ApiKeyInputDTO, UserId) {
    type Id = KeyId;
    async fn insert<'a, A>(&self, conn: A) -> Result<Self::Id, crate::repo::RepositoryError>
    where
        A: Acquire<'a, Database = Postgres>,
    {
        let mut executor = conn.acquire().await?;
        let (dto, id) = self;
        sqlx::query_scalar!(
            r#"
            INSERT INTO app.api_keys (user_id, name, key_hash)
            VALUES ($1 , $2, $3)

            RETURNING id as "id: KeyId"
            "#,
            id.0,
            dto.name,
            dto.key_hash.0
        )
        .fetch_one(&mut *executor)
        .await
        .map_err(Into::into)
    }
}
