use domain::enums::LayerCategory;
use domain::{DataProviderLayerId, UserId};
use serde_json::json;

use crate::{
    handlers::api::data_provider_layers::DataProviderLayerInputPayload, repo::traits::Insert,
};

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
        let enabled = dto.enabled.unwrap_or(true);
        let sort_order = dto.sort_order.unwrap_or(0);
        let res = sqlx::query!(
            "INSERT INTO app.data_provider_layers(
                service_id, name, abbreviation, source, category, description,
                enabled, style_config, display_options, country_code, subdivision,
                sort_order, added_by, last_updated_by
             ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $13)
             RETURNING id",
            dto.service_id.0,
            dto.name,
            dto.abbreviation,
            source as _,
            category as _,
            dto.description,
            enabled,
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
