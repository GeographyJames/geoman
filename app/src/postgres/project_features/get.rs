use domain::{FeatureId, enums::GeometryType};
use sqlx::PgPool;

use crate::repo::RepositoryError;

pub struct ProjectFeatureDownload {
    pub name: String,
    pub geom: Vec<u8>,
    pub srid: i32,
    pub geom_type: GeometryType,
}

pub async fn get_project_feature_for_download(
    pool: &PgPool,
    feature_id: FeatureId,
    project_slug: &str,
    collection_slug: &str,
) -> Result<Option<ProjectFeatureDownload>, RepositoryError> {
    sqlx::query_as!(
        ProjectFeatureDownload,
        r#"
        SELECT
            pf.name,
            ST_AsBinary(geom) AS "geom!",
            ST_Srid(geom) AS "srid!",
            GeometryType(geom) AS "geom_type!: GeometryType"
        FROM app.project_features pf
        JOIN app.projects p ON p.id = pf.project_id
        JOIN app.collections c ON c.id = pf.collection_id
        WHERE pf.id = $1
        AND p.slug = $2
        AND c.slug = $3
        "#,
        feature_id.0,
        project_slug,
        collection_slug,
    )
    .fetch_optional(pool)
    .await
    .map_err(RepositoryError::from)
}
