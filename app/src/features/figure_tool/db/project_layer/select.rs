use sqlx::Acquire;

use crate::{
    features::figure_tool::dtos::ProjectLayerOutputDTO,
    repo::{RepositoryError, traits::SelectAllWithParams},
};
use domain::ProjectId;

impl SelectAllWithParams for ProjectLayerOutputDTO {
    type Params<'a> = ProjectId;
    type MetaData<'a> = ();

    async fn select_all_with_params<'a, A>(
        executor: A,
        project_id: ProjectId,
    ) -> Result<(Vec<Self>, Self::MetaData<'a>), RepositoryError>
    where
        Self: Sized,
        A: Acquire<'a, Database = sqlx::Postgres>,
    {
        let mut conn = executor.acquire().await?;
        let layers: Vec<ProjectLayerOutputDTO> = sqlx::query_as(
            "SELECT
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
             AND t.tablename ~ '^p[0-9]{4}[ _][a-zA-Z0-9_ -]+$'
             AND g.f_geometry_column = 'geom'
             AND g.srid IN (27700, 4326)",
        )
        .fetch_all(&mut *conn)
        .await?;

        Ok((
            layers
                .into_iter()
                .filter(|l| l.project_id == project_id)
                .collect(),
            (),
        ))
    }
}
