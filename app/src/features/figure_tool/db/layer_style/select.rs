use sqlx::PgPool;

use crate::{
    app::features::figure_tool::{dtos::LayerStyleOutputDTO, ids::LayerStyleId},
    repo::SelectAll,
};

impl<'a> SelectAll<&'a PgPool> for LayerStyleOutputDTO {
    async fn select_all(executor: &'a PgPool) -> Result<Vec<Self>, crate::repo::RepositoryError>
    where
        Self: Sized,
    {
        sqlx::query_as!(
            LayerStyleOutputDTO,
            r#"
SELECT id as "id: LayerStyleId",
       f_table_schema as schema_name,
       f_table_name as table_name,
       stylename,
       description,
       type as geometry_type,
       useasdefault as use_as_default,
       owner

        FROM layer_styles
"#
        )
        .fetch_all(executor)
        .await
        .map_err(|e| e.into())
    }
}
