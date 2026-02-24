use domain::TeamId;

use crate::{postgres::PostgresRepo, repo::RepositoryError};

impl PostgresRepo {
    pub async fn delete_team(&self, id: TeamId) -> Result<(), RepositoryError> {
        sqlx::query!("DELETE FROM app.teams WHERE id = $1", id.0)
            .execute(&self.db_pool)
            .await?;
        Ok(())
    }
}
