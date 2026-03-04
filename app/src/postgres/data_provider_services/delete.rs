use domain::DataProviderServiceId;

use crate::{postgres::PostgresRepo, repo::RepositoryError};

impl PostgresRepo {
    pub async fn delete_data_provider_service(
        &self,
        id: DataProviderServiceId,
    ) -> Result<(), RepositoryError> {
        sqlx::query!(
            "DELETE FROM app.data_provider_services WHERE id = $1",
            id.0
        )
        .execute(&self.db_pool)
        .await?;
        Ok(())
    }
}
