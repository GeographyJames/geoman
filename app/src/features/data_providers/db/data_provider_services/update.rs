use domain::UserId;
use sqlx::{Acquire, Postgres};

use crate::{
    features::data_providers::{
        handlers::DataProviderServiceUpdatePayload, types::DataProviderServiceId,
    },
    repo::traits::Update,
};

impl Update
    for (
        DataProviderServiceUpdatePayload,
        DataProviderServiceId,
        UserId,
    )
{
    type Id = DataProviderServiceId;

    async fn update<'a, A>(&self, conn: A) -> Result<Self::Id, crate::repo::RepositoryError>
    where
        Self: Sized,
        A: Acquire<'a, Database = Postgres>,
    {
        let mut executor = conn.acquire().await?;
        let (payload, id, user_id) = self;
        let res = sqlx::query!(
            r#"
            UPDATE app.data_provider_services
            SET name = COALESCE($1, name),
                service_type = COALESCE($2, service_type),
                base_url = COALESCE($3, base_url),
                last_updated = NOW(),
                last_updated_by = $5
            WHERE id = $4
            RETURNING id
            "#,
            payload.name,
            payload.service_type as _,
            payload.base_url,
            id.0,
            user_id.0
        )
        .fetch_one(&mut *executor)
        .await?;
        Ok(DataProviderServiceId(res.id))
    }
}
