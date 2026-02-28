use domain::FeatureId;
use sqlx::PgPool;

use crate::repo::RepositoryError;

pub struct TurbineCsvRow {
    pub layout_name: String,
    pub id: i32,
    pub turbine_number: i32,
    pub hub_height_m: Option<f64>,
    pub rotor_diameter_m: Option<f64>,
    pub srid: i32,
    pub x: f64,
    pub y: f64,
}

pub async fn get_turbine_layout_csv(
    pool: &PgPool,
    layout_id: FeatureId,
    project_slug: &str,
    collection_slug: &str,
) -> Result<Vec<TurbineCsvRow>, RepositoryError> {
    sqlx::query_as!(
        TurbineCsvRow,
        r#"
        SELECT
            tl.name AS layout_name,
            t.id,
            t.turbine_number,
            t.hub_height_mm::float8 / 1000.0 AS hub_height_m,
            t.rotor_diameter_mm::float8 / 1000.0 AS rotor_diameter_m,
            ST_Srid(t.geom) AS "srid!",
            ST_X(t.geom) AS "x!",
            ST_Y(t.geom) AS "y!"
        FROM app.turbines t
        JOIN app.turbine_layouts tl ON tl.id = t.layout_id
        JOIN app.projects p ON p.id = tl.project_id
        JOIN app.collections c ON c.id = -1
        WHERE tl.id = $1
          AND p.slug = $2
          AND c.slug = $3
        ORDER BY t.turbine_number
        "#,
        layout_id.0,
        project_slug,
        collection_slug,
    )
    .fetch_all(pool)
    .await
    .map_err(RepositoryError::from)
}
