use domain::UserId;

use crate::{postgres::PostgresRepo, repo::RepositoryError};

impl PostgresRepo {
    pub async fn delete_user(&self, id: UserId) -> Result<(), RepositoryError> {
        sqlx::query!("DELETE FROM app.users WHERE id = $1", id.0)
            .execute(&self.db_pool)
            .await?;
        Ok(())
    }
}
