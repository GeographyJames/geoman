use domain::{DataProviderServiceId, UserId};

use crate::{
    handlers::api::data_provider_services::DataProviderServiceInputPayload, repo::traits::Insert,
};

impl Insert for (DataProviderServiceInputPayload, UserId) {
    type Id = DataProviderServiceId;

    async fn insert<'a, A>(&self, executor: A) -> Result<Self::Id, crate::repo::RepositoryError>
    where
        Self: Sized,
        A: sqlx::Acquire<'a, Database = sqlx::Postgres>,
    {
        let mut conn = executor.acquire().await?;
        let (dto, user) = self;
        let res = sqlx::query!(
            "INSERT INTO app.data_provider_services(provider_id, name, service_type, base_url, description, added_by, last_updated_by)
             VALUES ($1, $2, $3, $4, $5, $6, $6) RETURNING id",
            dto.provider_id.0,
            dto.name,
            dto.service_type as _,
            dto.base_url,
            dto.description,
            user.0
        )
        .fetch_one(&mut *conn)
        .await?;
        Ok(DataProviderServiceId(res.id))
    }
}
