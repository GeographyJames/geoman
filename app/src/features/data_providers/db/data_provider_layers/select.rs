use crate::features::data_providers::types::{
    DataProviderLayer, DataProviderLayerId, DataProviderServiceId, LayerCategory,
};

use crate::repo::traits::SelectAll;

impl SelectAll for DataProviderLayer {
    async fn select_all<'a, E>(executor: &'a E) -> Result<Vec<Self>, crate::repo::RepositoryError>
    where
        Self: Sized,
        &'a E: sqlx::PgExecutor<'a>,
    {
        let res = sqlx::query_as!(
            DataProviderLayer,
            r#"
            SELECT id as "id: DataProviderLayerId",
                   service_id as "service_id: DataProviderServiceId",
                   name,
                   abbreviation,
                   source as "source: serde_json::Value",
                   category as "category: LayerCategory",
                   description,
                   enabled_geoman,
                   enabled_figure_tool,
                   style_config as "style_config: serde_json::Value",
                   display_options as "display_options: serde_json::Value",
                   country_code,
                   subdivision,
                   sort_order
            FROM app.data_provider_layers
            ORDER BY sort_order ASC, name ASC
            "#
        )
        .fetch_all(executor)
        .await?;
        Ok(res)
    }
}
