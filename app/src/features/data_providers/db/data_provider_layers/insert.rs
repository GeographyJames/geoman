use domain::UserId;
use serde_json::json;

use crate::features::data_providers::handlers::DataProviderLayerInputPayload;
use crate::features::data_providers::types::{DataProviderLayerId, LayerCategory};
use crate::repo::traits::Insert;

impl Insert for (DataProviderLayerInputPayload, UserId) {
    type Id = DataProviderLayerId;

    async fn insert<'a, A>(&self, executor: A) -> Result<Self::Id, crate::repo::RepositoryError>
    where
        Self: Sized,
        A: sqlx::Acquire<'a, Database = sqlx::Postgres>,
    {
        let mut conn = executor.acquire().await?;
        let (dto, user) = self;
        let category = dto.category.clone().unwrap_or(LayerCategory::Overlay);
        let source = serde_json::to_value(&dto.source).unwrap_or_default();
        let style_config = dto.style_config.clone().unwrap_or_else(|| json!({}));
        let display_options = dto.display_options.clone().unwrap_or_else(|| json!({}));
        let enabled_geoman = dto.enabled.unwrap_or(true);
        let sort_order = dto.sort_order.unwrap_or(0);
        let slug = slug::slugify(&dto.name);
        let res = sqlx::query!(
            "INSERT INTO app.data_provider_layers(
                service_id, name, slug, abbreviation, source, category, description,
                enabled_geoman, style_config, display_options, country_code, subdivision,
                sort_order, added_by, last_updated_by
             ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $14)
             RETURNING id",
            dto.service_id.0,
            dto.name,
            slug,
            dto.abbreviation,
            source as _,
            category as _,
            dto.description,
            enabled_geoman,
            style_config as _,
            display_options as _,
            dto.country_code,
            dto.subdivision,
            sort_order,
            user.0
        )
        .fetch_one(&mut *conn)
        .await?;
        Ok(DataProviderLayerId(res.id))
    }
}
