use clerk_rs::validators::authorizer::ClerkJwt;
use domain::{KeyHash, UserId};

use crate::repo::{
    RepositoryError,
    traits::{SelectOne, SelectOneWithParams},
    user_id::SelectOneParams,
};

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

impl SelectOneWithParams<&KeyHash> for UserId {
    type Params<'a> = &'a SelectOneParams;
    async fn select_one_with_params<'a, E>(
        executor: &'a E,
        key_hash: &KeyHash,
        params: Self::Params<'a>,
    ) -> Result<Option<Self>, crate::repo::RepositoryError>
    where
        &'a E: sqlx::PgExecutor<'a>,
    {
        sqlx::query_scalar!(
            r#"UPDATE app.api_keys
                SET last_used = NOW(),
                    last_used_ip = $1,
                    last_used_user_agent = $2
              WHERE key_hash = $3
                AND revoked IS NULL
                AND expiry > NOW()
          RETURNING user_id AS "user_id: UserId""#,
            params.ip_address as _,
            params.user_agent,
            key_hash.0
        )
        .fetch_optional(executor)
        .await
        .map_err(Into::into)
    }
}
