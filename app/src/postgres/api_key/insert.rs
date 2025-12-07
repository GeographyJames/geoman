use domain::{ApiKeyInputDTO, KeyId};

use crate::repo::traits::Insert;

impl Insert for (ApiKeyInputDTO, &str) {
    type Id = KeyId;
    async fn insert<'a, E>(&self, executor: &'a E) -> Result<Self::Id, crate::repo::RepositoryError>
    where
        &'a E: sqlx::PgExecutor<'a>,
    {
        let (dto, auth_id) = self;
        sqlx::query_scalar!(
            r#"
            INSERT INTO app.api_keys (user_id, name, key_hash)
            SELECT user_id, $2, $3
            FROM app.upsert_user_and_get_context($1)
            RETURNING id as "id: KeyId"
            "#,
            auth_id,
            dto.name,
            dto.key_hash.0
        )
        .fetch_one(executor)
        .await
        .map_err(Into::into)
    }
}
