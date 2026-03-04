use domain::{DataProviderId, UserId};

use crate::{handlers::api::data_providers::DataProviderInputPayload, repo::traits::Insert};

impl Insert for (DataProviderInputPayload, UserId) {
    type Id = DataProviderId;

    async fn insert<'a, A>(&self, executor: A) -> Result<Self::Id, crate::repo::RepositoryError>
    where
        Self: Sized,
        A: sqlx::Acquire<'a, Database = sqlx::Postgres>,
    {
        let mut conn = executor.acquire().await?;
        let (dto, user) = self;
        let res = sqlx::query!(
            "INSERT INTO app.data_providers(name, description, country_code, subdivision, added_by, last_updated_by)
             VALUES ($1, $2, $3, $4, $5, $5) RETURNING id",
            dto.name,
            dto.description,
            dto.country_code,
            dto.subdivision,
            user.0
        )
        .fetch_one(&mut *conn)
        .await?;
        Ok(DataProviderId(res.id))
    }
}
