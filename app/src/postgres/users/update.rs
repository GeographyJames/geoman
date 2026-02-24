use domain::{TeamId, UserId, UserInputDto};
use sqlx::{Acquire, Postgres};

use crate::{AuthenticatedUser, handlers::api::users::PatchUserPayload, repo::traits::Update};

impl<'b> Update for UserInputDto<'b> {
    type Id = AuthenticatedUser;

    async fn update<'a, E>(&self, conn: E) -> Result<Self::Id, crate::repo::RepositoryError>
    where
        E: Acquire<'a, Database = Postgres>,
    {
        let mut executor = conn.acquire().await?;
        sqlx::query_as!(
            AuthenticatedUser,
            r#"UPDATE app.users SET
          first_name = $1, last_name = $2, username = $3
          WHERE clerk_id = $4  
             RETURNING id AS "id: UserId", team_id AS "team_id: TeamId", admin, first_name, last_name, username"#,

            self.first_name.unwrap_or("Unknown"),
            self.last_name.unwrap_or("User"),
            self.username,
            self.auth_id

        )
        .fetch_one(&mut *executor)
        .await
        .map_err(Into::into)
    }
}

impl Update for (PatchUserPayload, UserId) {
    type Id = UserId;

    async fn update<'a, A>(&self, conn: A) -> Result<Self::Id, crate::repo::RepositoryError>
    where
        Self: Sized,
        A: Acquire<'a, Database = Postgres>,
    {
        let mut executor = conn.acquire().await?;
        let (dto, user_id) = self;
        let res = sqlx::query!(
            r#"
        UPDATE app.users
        SET team_id = COALESCE($1, team_id),
            admin = COALESCE($2, admin)
        WHERE id = $3 RETURNING id"#,
            dto.team_id.map(|id| id.0),
            dto.admin,
            user_id.0
        )
        .fetch_one(&mut *executor)
        .await?;
        Ok(UserId(res.id))
    }
}
