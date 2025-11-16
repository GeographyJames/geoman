use domain::ProjectFeature;
use futures::{Stream, StreamExt};
use geojson::Geometry;
use ogc::types::features::Query;
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
            _ => {
                return Err(RepositoryError::UnexpectedError(anyhow::anyhow!(
                    "invalid feature properties"
                )));
            }
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

#[derive(Clone)]
#[non_exhaustive]
pub struct SelectAllParams {
    pub limit: Option<usize>,
    pub slug: String,
}

impl SelectAllParams {
    pub fn from_query(query: Query, slug: String) -> Self {
        SelectAllParams {
            limit: query.limit,
            slug,
        }
    }
}

impl SelectAllWithParamsStreaming for ProjectFeature {
    type Params = SelectAllParams;

    fn select_all_with_params_streaming(
        executor: PoolWrapper,
        params: Self::Params,
    ) -> impl Stream<Item = Result<Self, RepositoryError>> + use<> {
        sqlx::query_scalar!(
            r#"
            SELECT jsonb_build_object(
            'id', f.id,
            'geometry', ST_AsGeoJSON(ST_Transform(fo.geom, 4326))::jsonb,
            'is_primary', f.is_primary,
            'name', f.name,
            'properties', f.properties 
        )
            as "feature!: Json<ProjectFeature>"
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
        .fetch(executor)
        .map(|res| res.map_err(RepositoryError::from).map(|json| json.0))
    }
}
