use domain::{DataProviderId, UserId};
use sqlx::{Acquire, Postgres};

use crate::{handlers::api::data_providers::DataProviderUpdatePayload, repo::traits::Update};

impl Update for (DataProviderUpdatePayload, DataProviderId, UserId) {
    type Id = DataProviderId;

    async fn update<'a, A>(&self, conn: A) -> Result<Self::Id, crate::repo::RepositoryError>
    where
        Self: Sized,
        A: Acquire<'a, Database = Postgres>,
    {
        let mut executor = conn.acquire().await?;
        let (payload, id, user_id) = self;
        let res = sqlx::query!(
            r#"
            UPDATE app.data_providers
            SET name = COALESCE($1, name),
                description = CASE WHEN $2 THEN $3 ELSE description END,
                country_code = CASE WHEN $4 THEN $5 ELSE country_code END,
                subdivision = CASE WHEN $6 THEN $7 ELSE subdivision END,
                last_updated = NOW(),
                last_updated_by = $9
            WHERE id = $8
            RETURNING id
            "#,
            payload.name,
            payload.description.is_some(),
            payload.description.clone().flatten(),
            payload.country_code.is_some(),
            payload.country_code.clone().flatten(),
            payload.subdivision.is_some(),
            payload.subdivision.clone().flatten(),
            id.0,
            user_id.0
        )
        .fetch_one(&mut *executor)
        .await?;
        Ok(DataProviderId(res.id))
    }
}
