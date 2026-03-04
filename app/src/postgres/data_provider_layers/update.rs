use domain::{DataProviderLayerId, UserId};
use sqlx::{Acquire, Postgres};

use crate::{
    handlers::api::data_provider_layers::DataProviderLayerUpdatePayload, repo::traits::Update,
};

impl Update for (DataProviderLayerUpdatePayload, DataProviderLayerId, UserId) {
    type Id = DataProviderLayerId;

    async fn update<'a, A>(&self, conn: A) -> Result<Self::Id, crate::repo::RepositoryError>
    where
        Self: Sized,
        A: Acquire<'a, Database = Postgres>,
    {
        let mut executor = conn.acquire().await?;
        let (payload, id, user_id) = self;
        let res = sqlx::query!(
            r#"
            UPDATE app.data_provider_layers
            SET name = COALESCE($1, name),
                abbreviation = CASE WHEN $2 THEN $3 ELSE abbreviation END,
                source = COALESCE($4, source),
                category = COALESCE($5, category),
                description = CASE WHEN $6 THEN $7 ELSE description END,
                enabled = COALESCE($8, enabled),
                style_config = COALESCE($9, style_config),
                display_options = COALESCE($10, display_options),
                country_code = CASE WHEN $11 THEN $12 ELSE country_code END,
                subdivision = CASE WHEN $13 THEN $14 ELSE subdivision END,
                sort_order = COALESCE($15, sort_order),
                last_updated = NOW(),
                last_updated_by = $17
            WHERE id = $16
            RETURNING id
            "#,
            payload.name,
            payload.abbreviation.is_some(),
            payload.abbreviation.clone().flatten(),
            payload.source as _,
            payload.category as _,
            payload.description.is_some(),
            payload.description.clone().flatten(),
            payload.enabled,
            payload.style_config as _,
            payload.display_options as _,
            payload.country_code.is_some(),
            payload.country_code.clone().flatten(),
            payload.subdivision.is_some(),
            payload.subdivision.clone().flatten(),
            payload.sort_order,
            id.0,
            user_id.0
        )
        .fetch_one(&mut *executor)
        .await?;
        Ok(DataProviderLayerId(res.id))
    }
}
