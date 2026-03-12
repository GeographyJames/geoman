use sqlx::PgConnection;

use crate::{
    features::figure_tool::{dtos::BaseMapOutputDTO, ids::BaseMapId},
    repo::RepositoryError,
};

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

impl BaseMapOutputDTO {
    pub async fn select(conn: &mut PgConnection, id: &BaseMapId) -> Result<Self, RepositoryError> {
        let res = sqlx::query_as(&format!("{BASE_QUERY} WHERE bm.id = $1"))
            .bind(id.as_ref())
            .fetch_one(conn)
            .await?;
        Ok(res)
    }
}
