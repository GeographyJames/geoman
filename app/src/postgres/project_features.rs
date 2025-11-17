use domain::{ProjectFeature, ProjectId};
use futures::{Stream, StreamExt};
use geojson::Geometry;
use serde::Deserialize;
use serde_json::Value;
use sqlx::types::Json;

use crate::{
    errors::RepositoryError,
    postgres::{
        PoolWrapper,
        traits::{SelectAllWithParamsStreaming, SelectOne},
    },
};

#[derive(Clone)]
#[non_exhaustive]
pub struct SelectAllParams {
    pub limit: Option<usize>,
    pub slug: String,
    pub project_id: Option<ProjectId>,
}

#[derive(Deserialize)]
struct ProjectFeatureRow {
    pub id: i32,
    pub properties: serde_json::Value,
    pub name: String,
    pub geometry: Json<geojson::Geometry>,
    pub is_primary: bool,
}

impl TryInto<ProjectFeature> for ProjectFeatureRow {
    type Error = RepositoryError;
    fn try_into(self) -> Result<ProjectFeature, RepositoryError> {
        let Self {
            id,
            properties,
            name,
            geometry,
            is_primary,
        } = self;
        let properties = match properties {
            Value::Object(map) => map,
            _ => serde_json::Map::default(),
        };
        Ok(ProjectFeature {
            id,
            properties,
            name,
            geometry: geometry.0,
            is_primary,
        })
    }
}

impl SelectOne for ProjectFeature {
    type Id<'a> = i32;
    async fn select_one<'a, 'e, E>(
        executor: E,
        id: Self::Id<'a>,
    ) -> Result<Option<Self>, RepositoryError>
    where
        E: sqlx::PgExecutor<'e>,
    {
        sqlx::query_as!(
            ProjectFeatureRow,
            r#"
            SELECT f.id,
                f.name,
                f.is_primary,
                ST_AsGeoJSON(ST_Transform(fo.geom, 4326))::jsonb as "geometry!: Json<Geometry>",
                f.properties
            FROM app.project_features f
            JOIN app.feature_objects fo ON fo.project_feature_id = f.id
            WHERE f.id = $1
            "#,
            id
        )
        .fetch_optional(executor)
        .await?
        .map(|row| row.try_into())
        .transpose()
    }
}

impl SelectAllParams {
    pub fn from_query(query: ogc::features::Query, slug: String) -> Self {
        SelectAllParams {
            limit: query.limit,
            slug,
            project_id: None,
        }
    }
}

impl SelectAllWithParamsStreaming for ProjectFeature {
    type Params = SelectAllParams;

    fn select_all_with_params_streaming(
        executor: PoolWrapper,
        params: Self::Params,
    ) -> impl Stream<Item = Result<Self, RepositoryError>> + use<> {
        let query = match params.project_id {
            Some(id) => sqlx::query_as!(
                ProjectFeatureRow,
                r#"
            SELECT 
                f.id,
                ST_AsGeoJSON(ST_Transform(fo.geom, 4326))::jsonb as "geometry!: Json<Geometry>",
                f.is_primary,
                f.name,
                f.properties 
            FROM app.project_features f
            JOIN app.collections c ON c.id = f.collection_id
            JOIN app.feature_objects fo ON fo.project_feature_id = f.id
            WHERE c.slug = $1
            AND status = 'ACTIVE'
            AND f.project_id = $2
            ORDER BY f.id
            LIMIT $3
            "#,
                params.slug,
                id.0,
                params.limit.map(|l| l as i64)
            )
            .fetch(executor),
            None => sqlx::query_as!(
                ProjectFeatureRow,
                r#"
            SELECT 
                f.id,
                ST_AsGeoJSON(ST_Transform(fo.geom, 4326))::jsonb as "geometry!: Json<Geometry>",
                f.is_primary,
                f.name,
                f.properties 
            FROM app.project_features f
            JOIN app.collections c ON c.id = f.collection_id
            JOIN app.feature_objects fo ON fo.project_feature_id = f.id
            WHERE c.slug = $1 AND status = 'ACTIVE'
            ORDER BY f.id
            LIMIT $2
            "#,
                params.slug,
                params.limit.map(|l| l as i64)
            )
            .fetch(executor),
        };

        query.map(|res| res?.try_into())
    }
}

#[cfg(test)]
mod tests {
    use domain::ProjectFeature;
    use serde_json::json;

    use crate::postgres::project_features::ProjectFeatureRow;

    #[test]
    fn project_feature_row_converts_to_project_feature() {
        let row = ProjectFeatureRow {
            id: 0,
            properties: json!("{}"),
            name: uuid::Uuid::new_v4().to_string(),
            geometry: sqlx::types::Json(geojson::Geometry::new(geojson::Value::Point(vec![
                1., 1.,
            ]))),
            is_primary: true,
        };
        let _feature: ProjectFeature = row.try_into().expect("failed to convert row to feature");
    }
}
