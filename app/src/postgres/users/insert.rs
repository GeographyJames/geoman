use domain::UserInputDto;

use crate::{AuthenticatedUser, constants::USER_AUTH_ID_COLUMN, repo::traits::Insert};

impl Insert for UserInputDto {
    type Id = AuthenticatedUser;

    async fn insert<'a, E>(&self, executor: &'a E) -> Result<Self::Id, crate::repo::RepositoryError>
    where
        &'a E: sqlx::PgExecutor<'a>,
    {
        sqlx::query_as(&format!(
            "INSERT INTO app.users (
            {USER_AUTH_ID_COLUMN}, first_name, last_name, username, team_id
            ) VALUES ($1, $2, $3, $4, -1)
             RETURNING id, team_id, admin",
        ))
        .bind(&self.auth_id)
        .bind(&self.first_name)
        .bind(&self.last_name)
        .bind(&self.username)
        .fetch_one(executor)
        .await
        .map_err(Into::into)
    }
}
