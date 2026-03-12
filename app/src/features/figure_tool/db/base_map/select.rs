use sqlx::PgConnection;

use crate::{
    features::figure_tool::{dtos::BaseMapOutputDTO, ids::BaseMapId},
    repo::RepositoryError,
};

const BASE_QUERY: &str = r#"SELECT dpl.id,
           dpl.name,
           dpl.slug,
           dpl.figure_default_main_map_base_map AS default_main_map_base_map,
           dpl.figure_default_overview_map_base_map AS default_overview_map_base_map,
           dp.id AS "dp_id",
           dp.name AS "dp_name",
           dp.copyright_text,
           dpl.source AS datasource
    FROM app.data_provider_layers dpl
    JOIN app.data_provider_services dps ON dpl.service_id = dps.id
    JOIN app.data_providers dp ON dps.provider_id = dp.id
    WHERE dpl.category = 'basemap'"#;

impl BaseMapOutputDTO {
    pub async fn select(conn: &mut PgConnection, id: &BaseMapId) -> Result<Self, RepositoryError> {
        let res = sqlx::query_as(&format!("{BASE_QUERY} AND dpl.id = $1"))
            .bind(id.as_ref())
            .fetch_one(conn)
            .await?;
        Ok(res)
    }
}
