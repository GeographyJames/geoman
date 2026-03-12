use sqlx::{PgConnection, PgExecutor};

use crate::{
    app::features::figure_tool::{dtos::base_map::BaseMapOutputDTO, ids::BaseMapId},
    repo::{Select, SelectAll},
};

impl<REPO> SelectAll<REPO> for BaseMapOutputDTO
where
    for<'a> REPO: PgExecutor<'a>,
{
    async fn select_all(executor: REPO) -> Result<Vec<Self>, crate::repo::RepositoryError> {
        let res = sqlx::query_as(BASE_QUERY).fetch_all(executor).await?;
        Ok(res)
    }
}

const BASE_QUERY: &str = r#"SELECT bm.id,
           bm.name,
           slug,
           default_main_map_base_map,
           default_overview_map_base_map,
           dp.id AS "dp_id",
           dp.name AS "dp_name",
           dp.copyright_text,
           datasource

    FROM app.base_maps bm
     JOIN app.base_map_data_providers dp ON bm.data_provider_id = dp.id"#;

impl<'a> Select<&'a mut PgConnection, BaseMapId> for BaseMapOutputDTO {
    async fn select(
        repository: &'a mut PgConnection,
        id: &BaseMapId,
    ) -> Result<Self, crate::repo::RepositoryError>
    where
        Self: Sized,
    {
        let res = sqlx::query_as(&format!("{BASE_QUERY} WHERE bm.id = $1"))
            .bind(id.as_ref())
            .fetch_one(repository)
            .await?;
        Ok(res)
    }
}
