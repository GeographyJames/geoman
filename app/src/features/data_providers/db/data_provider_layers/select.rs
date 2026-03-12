use crate::features::data_providers::types::{
    DataProviderLayer, DataProviderLayerId, DataProviderServiceId, LayerCategory, LayerSource,
};

use crate::repo::traits::SelectAll;

impl SelectAll for DataProviderLayer {
    async fn select_all<'a, A>(executor: A) -> Result<Vec<Self>, crate::repo::RepositoryError>
    where
        Self: Sized,
        A: sqlx::Acquire<'a, Database = sqlx::Postgres>,
    {
        let mut conn = executor.acquire().await?;
        let res = sqlx::query_as!(
            DataProviderLayer,
            r#"
            SELECT id as "id: DataProviderLayerId",
                   service_id as "service_id: DataProviderServiceId",
                   name,
                   abbreviation,
                   source as "source: sqlx::types::Json<LayerSource>",
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
        .fetch_all(&mut *conn)
        .await?;
        Ok(res)
    }
}
