use domain::{TeamId, UserId, UserInputDto};
use sqlx::{Acquire, Postgres};

use crate::{AuthenticatedUser, repo::traits::Insert};

impl<'b> Insert for UserInputDto<'b> {
    type Id = AuthenticatedUser;

    async fn insert<'a, A>(&self, conn: A) -> Result<Self::Id, crate::repo::RepositoryError>
    where
        A: Acquire<'a, Database = Postgres>,
    {
        let mut executor = conn.acquire().await?;
        sqlx::query_as!(
            AuthenticatedUser,
            r#"INSERT INTO app.users (
            clerk_id, first_name, last_name, username, team_id
            ) VALUES ($1, $2, $3, $4, -1)
             RETURNING id AS "id: UserId", team_id AS "team_id: TeamId", admin, first_name, last_name, username"#,
            self.auth_id,
            self.first_name.unwrap_or("Unknown"),
            self.last_name.unwrap_or("User"),
            self.username
        )
        .fetch_one(&mut *executor)
        .await
        .map_err(Into::into)
    }
}
