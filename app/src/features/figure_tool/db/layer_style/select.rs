use sqlx::Acquire;

use crate::{
    features::figure_tool::{dtos::LayerStyleOutputDTO, ids::LayerStyleId},
    repo::{RepositoryError, traits::SelectAll},
};

impl SelectAll for LayerStyleOutputDTO {
    async fn select_all<'a, A>(executor: A) -> Result<Vec<Self>, RepositoryError>
    where
        Self: Sized,
        A: Acquire<'a, Database = sqlx::Postgres>,
    {
        let mut conn = executor.acquire().await?;
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
        .fetch_all(&mut *conn)
        .await
        .map_err(|e| e.into())
    }
}
