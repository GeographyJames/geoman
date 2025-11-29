use clerk_rs::validators::authorizer::ClerkJwt;
use domain::{KeyHash, UserId};

use crate::repo::{RepositoryError, traits::SelectOne};

impl SelectOne<&ClerkJwt> for UserId {
    async fn select_one<'a, E>(
        executor: &'a E,
        token: &ClerkJwt,
    ) -> Result<Option<Self>, crate::repo::RepositoryError>
    where
        &'a E: sqlx::PgExecutor<'a>,
    {
        sqlx::query_scalar!(
            r#"SELECT id as "id:UserId" FROM app.users WHERE clerk_id = $1"#,
            token.sub
        )
        .fetch_optional(executor)
        .await
        .map_err(RepositoryError::from)
    }
}

impl SelectOne<&KeyHash> for UserId {
    async fn select_one<'a, E>(
        executor: &'a E,
        key_hash: &KeyHash,
    ) -> Result<Option<Self>, crate::repo::RepositoryError>
    where
        &'a E: sqlx::PgExecutor<'a>,
    {
        sqlx::query_scalar!(
            r#"UPDATE app.api_keys
                SET last_used = NOW()
              WHERE key_hash = $1
                AND revoked = false
                AND expiry > NOW()
          RETURNING user_id AS "user_id: UserId""#,
            key_hash.0
        )
        .fetch_optional(executor)
        .await
        .map_err(Into::into)
    }
}
