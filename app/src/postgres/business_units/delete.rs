use domain::BusinessUnitId;

use crate::{postgres::PostgresRepo, repo::RepositoryError};

impl PostgresRepo {
    pub async fn delete_business_unit(&self, id: BusinessUnitId) -> Result<(), RepositoryError> {
        sqlx::query!("DELETE FROM app.business_units WHERE id = $1", id.0)
            .execute(&self.db_pool)
            .await?;
        Ok(())
    }
}
