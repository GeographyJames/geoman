use clerk_rs::validators::authorizer::ClerkJwt;
use domain::{KeyHash, User, UserId};

use crate::repo::{
    RepositoryError,
    traits::{SelectAll, SelectOne, SelectOneWithParams},
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

const SELECT_USER_QUERY: &str = r#"SELECT u.id, u.first_name, u.last_name, u.clerk_id,
             CASE
         WHEN t.id IS NULL THEN NULL
         ELSE ROW(t.id, t.name)::app.team
         END as team
         FROM app.users u
         LEFT JOIN app.teams t ON t.id = u.team_id"#;

impl SelectAll for User {
    async fn select_all<'a, E>(executor: &'a E) -> Result<Vec<Self>, RepositoryError>
    where
        Self: Sized,
        &'a E: sqlx::PgExecutor<'a>,
    {
        sqlx::query_as(SELECT_USER_QUERY)
            .fetch_all(executor)
            .await
            .map_err(Into::into)
    }
}

impl SelectOne<UserId> for User {
    async fn select_one<'a, E>(executor: &'a E, id: UserId) -> Result<Option<Self>, RepositoryError>
    where
        Self: Sized,
        &'a E: sqlx::PgExecutor<'a>,
    {
        sqlx::query_as(&format!("{} WHERE u.id = $1", SELECT_USER_QUERY))
            .bind(id.0)
            .fetch_optional(executor)
            .await
            .map_err(Into::into)
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
