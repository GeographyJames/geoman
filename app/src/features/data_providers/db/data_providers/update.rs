use domain::UserId;
use sqlx::{Acquire, Postgres};

use crate::{
    features::data_providers::{handlers::DataProviderUpdatePayload, types::DataProviderId},
    repo::traits::Update,
};

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
                country_code = CASE WHEN $2 THEN $3 ELSE country_code END,
                subdivision = CASE WHEN $4 THEN $5 ELSE subdivision END,
                last_updated = NOW(),
                last_updated_by = $7
            WHERE id = $6
            RETURNING id
            "#,
            payload.name,
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
