use domain::{KeyHash, Team, TeamId, User, UserId};

use crate::{
    AuthenticatedUser,
    repo::{
        RepositoryError,
        traits::{SelectAll, SelectOne, SelectOneWithParams},
        user_id::SelectOneParams,
    },
};

impl SelectAll for User {
    async fn select_all<'a, E>(executor: &'a E) -> Result<Vec<Self>, RepositoryError>
    where
        Self: Sized,
        &'a E: sqlx::PgExecutor<'a>,
    {
        sqlx::query_as!(
            User,
            r#"SELECT u.id AS "id: UserId",
                    u.first_name,
                    u.last_name,
                    u.clerk_id,
                    u.operating_country_code,
                    (ROW(t.id, t.name)::app.team) as "team!: Team"
                FROM app.users u JOIN app.teams t ON t.id = u.team_id "#,
        )
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
        sqlx::query_as!(
            User,
            r#"SELECT u.id AS "id: UserId",
                    u.first_name,
                    u.last_name,
                    u.clerk_id,
                    u.operating_country_code,
                    (ROW(t.id, t.name)::app.team) as "team!: Team"
                FROM app.users u JOIN app.teams t ON t.id = u.team_id
                WHERE u.id = $1 "#,
            id.0
        )
        .fetch_optional(executor)
        .await
        .map_err(Into::into)
    }
}

impl SelectOneWithParams<&KeyHash> for AuthenticatedUser {
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
            AuthenticatedUser,
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
                        u.admin,
                        u.first_name,
                        u.last_name,
                        u.username
                        "#,
            params.ip_address as _,
            params.user_agent,
            key_hash.0
        )
        .fetch_optional(executor)
        .await
        .map_err(Into::into)
    }
}

impl SelectOne<&str> for AuthenticatedUser {
    async fn select_one<'a, E>(
        executor: &'a E,
        token: &str,
    ) -> Result<Option<Self>, crate::repo::RepositoryError>
    where
        &'a E: sqlx::PgExecutor<'a>,
    {
        sqlx::query_as!(
            AuthenticatedUser,
            r#"SELECT id as "id: UserId", team_id as "team_id: TeamId", admin, username, first_name, last_name FROM app.users WHERE clerk_id = $1"#,
            token
        )
        .fetch_optional(executor)
        .await
        .map_err(RepositoryError::from)
    }
}
