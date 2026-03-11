use domain::UserId;

use crate::{
    features::data_providers::{handlers::DataProviderInputPayload, types::DataProviderId},
    repo::traits::Insert,
};

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
            "INSERT INTO app.data_providers(name, country_code, subdivision, added_by, last_updated_by)
             VALUES ($1, $2, $3, $4, $4) RETURNING id",
            dto.name,
            dto.country_code,
            dto.subdivision,
            user.0
        )
        .fetch_one(&mut *conn)
        .await?;
        Ok(DataProviderId(res.id))
    }
}
