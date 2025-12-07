use domain::UserInputDto;

use crate::{constants::USER_AUTH_ID_COLUMN, repo::traits::Insert, types::UserContext};

impl Insert for UserInputDto {
    type Id = UserContext;

    async fn insert<'a, E>(&self, executor: &'a E) -> Result<Self::Id, crate::repo::RepositoryError>
    where
        &'a E: sqlx::PgExecutor<'a>,
    {
        sqlx::query_as(&format!(
            "INSERT INTO app.users (
            {USER_AUTH_ID_COLUMN}, first_name, last_name
            ) VALUES ($1, $2, $3)
             RETURNING id, team_id, admin",
        ))
        .bind(&self.auth_id)
        .bind(&self.first_name)
        .bind(&self.last_name)
        .fetch_one(executor)
        .await
        .map_err(Into::into)
    }
}
