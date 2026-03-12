use sqlx::PgPool;

use crate::{
    app::features::figure_tool::{dtos::ProjectLayerOutputDTO, ids::ProjectId},
    repo::SelectAllForProject,
};

impl<'a> SelectAllForProject<&'a PgPool, ProjectId> for ProjectLayerOutputDTO {
    async fn select_all_for_project(
        executor: &'a PgPool,
        project_id: &ProjectId,
    ) -> Result<Vec<Self>, crate::repo::RepositoryError>
    where
        Self: Sized,
    {
        let layers: Vec<ProjectLayerOutputDTO> = sqlx::query_as(
            "  SELECT
       t.tablename AS table_name,
       t.schemaname,
      t.tableowner AS owner,
      g.type AS geometry_type,
      g.srid AS epsg_id
  FROM pg_tables t
  INNER JOIN geometry_columns g
      ON g.f_table_schema = t.schemaname
      AND g.f_table_name = t.tablename
  WHERE t.schemaname = 'project_data'
  AND    t.tablename ~ '^p[0-9]{4}[ _][a-zA-Z0-9_ -]+$'
 AND g.f_geometry_column = 'geom'
 AND g.srid IN (27700, 4326)",
        )
        .fetch_all(executor)
        .await?;
        Ok(layers
            .into_iter()
            .filter(|l| &l.project_id == project_id)
            .collect())
    }
}
