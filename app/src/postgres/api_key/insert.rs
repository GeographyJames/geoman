use domain::{ApiKeyInputDTO, KeyId, UserId};

use crate::repo::traits::Insert;

impl Insert for (&ApiKeyInputDTO, UserId) {
    type Id = KeyId;
    async fn insert<'a, E>(&self, executor: &'a E) -> Result<Self::Id, crate::repo::RepositoryError>
    where
        &'a E: sqlx::PgExecutor<'a>,
    {
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
        .fetch_one(executor)
        .await
        .map_err(Into::into)
    }
}
