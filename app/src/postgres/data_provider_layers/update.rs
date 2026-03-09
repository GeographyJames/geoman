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
        let slug = payload.name.as_deref().map(slug::slugify);
        let res = sqlx::query!(
            r#"
            UPDATE app.data_provider_layers
            SET name = COALESCE($1, name),
                slug = COALESCE($2, slug),
                abbreviation = CASE WHEN $3 THEN $4 ELSE abbreviation END,
                source = COALESCE($5, source),
                category = COALESCE($6, category),
                description = CASE WHEN $7 THEN $8 ELSE description END,
                enabled = COALESCE($9, enabled),
                style_config = COALESCE($10, style_config),
                display_options = COALESCE($11, display_options),
                country_code = CASE WHEN $12 THEN $13 ELSE country_code END,
                subdivision = CASE WHEN $14 THEN $15 ELSE subdivision END,
                sort_order = COALESCE($16, sort_order),
                last_updated = NOW(),
                last_updated_by = $18
            WHERE id = $17
            RETURNING id
            "#,
            payload.name,
            slug,
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
