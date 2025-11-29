use domain::{ApiKeyInputDTO, KeyId};

use crate::repo::traits::Insert;

impl Insert for ApiKeyInputDTO {
    type Id = KeyId;
    async fn insert<'a, E>(&self, executor: &'a E) -> Result<Self::Id, crate::repo::RepositoryError>
    where
        &'a E: sqlx::PgExecutor<'a>,
    {
        sqlx::query_scalar!(
            r#"INSERT INTO app.api_keys (user_id, name, key_hash) VALUES ($1, $2, $3) RETURNING id as "id:KeyId""#,
            self.user_id.0,
            self.name,
            self.key_hash.0
        ).fetch_one(executor).await.map_err(Into::into)
    }
}
