use domain::DataProviderId;

use crate::{postgres::PostgresRepo, repo::RepositoryError};

impl PostgresRepo {
    pub async fn delete_data_provider(&self, id: DataProviderId) -> Result<(), RepositoryError> {
        sqlx::query!("DELETE FROM app.data_providers WHERE id = $1", id.0)
            .execute(&self.db_pool)
            .await?;
        Ok(())
    }
}
