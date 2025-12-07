use domain::{KeyHash, TeamId, User, UserId};

use crate::{
    constants::USER_AUTH_ID_COLUMN,
    postgres::sql_fragments::{team_join_fragment, user_row_fragment},
    repo::{
        RepositoryError,
        traits::{SelectAll, SelectOne, SelectOneWithParams},
        user_id::SelectOneParams,
    },
    types::UserContext,
};

impl SelectOne<&str> for UserContext {
    async fn select_one<'a, E>(
        executor: &'a E,
        authentication_id: &str,
    ) -> Result<Option<Self>, crate::repo::RepositoryError>
    where
        &'a E: sqlx::PgExecutor<'a>,
    {
        sqlx::query_as(&format!(
            r#"SELECT id as TheId, team_id, admin FROM app.users WHERE {USER_AUTH_ID_COLUMN} = $1"#
        ))
        .bind(authentication_id)
        .fetch_optional(executor)
        .await
        .map_err(RepositoryError::from)
    }
}

impl SelectAll for User {
    async fn select_all<'a, E>(executor: &'a E) -> Result<Vec<Self>, RepositoryError>
    where
        Self: Sized,
        &'a E: sqlx::PgExecutor<'a>,
    {
        sqlx::query_scalar(&format!(
            "SELECT {} FROM app.users u {}",
            user_row_fragment("u", "user"),
            team_join_fragment("u")
        ))
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
        sqlx::query_scalar(&format!(
            "SELECT {}
            FROM app.users u
            {}
            WHERE u.id = $1",
            user_row_fragment("u", "user"),
            team_join_fragment("u")
        ))
        .bind(id.0)
        .fetch_optional(executor)
        .await
        .map_err(Into::into)
    }
}

impl SelectOneWithParams<&KeyHash> for UserContext {
    type Params<'a> = &'a SelectOneParams;
    async fn select_one_with_params<'a, E>(
        executor: &'a E,
        key_hash: &KeyHash,
        params: Self::Params<'a>,
    ) -> Result<Option<Self>, crate::repo::RepositoryError>
    where
        &'a E: sqlx::PgExecutor<'a>,
    {
        sqlx::query_as!(
            UserContext,
            r#"UPDATE app.api_keys k
                    SET last_used = NOW(),
                        last_used_ip = $1,
                        last_used_user_agent = $2
                   FROM app.users u
                  WHERE k.key_hash = $3
                    AND k.revoked IS NULL
                    AND k.expiry > NOW()
                    AND u.id = k.user_id
              RETURNING u.id AS "id: UserId",
                        u.team_id AS "team_id: TeamId",
                        u.admin"#,
            params.ip_address as _,
            params.user_agent,
            key_hash.0
        )
        .fetch_optional(executor)
        .await
        .map_err(Into::into)
    }
}
