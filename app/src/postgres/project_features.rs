use domain::{FeatureIdWithCollectionSlug, ProjectFeature, ProjectId, poject_feature::Properties};
use futures::{Stream, StreamExt};
use geojson::Geometry;
use serde::Deserialize;
use serde_json::Value;
use sqlx::types::Json;

use crate::{
    errors::RepositoryError,
    postgres::{
        PoolWrapper,
        traits::{SelectAllWithParamsStreaming, SelectOneWithParams},
    },
};

#[derive(Clone)]
pub struct SelectAllParams {
    pub limit: Option<usize>,
    pub slug: String,
    pub project_id: Option<ProjectId>,
    pub srid: Option<i32>,
}

#[derive(Clone)]
pub struct SelectOneParams {
    pub project_id: Option<ProjectId>,
    pub srid: Option<i32>,
}

impl SelectAllParams {
    pub fn from_query(query: &ogc::features::Query, slug: String) -> Self {
        SelectAllParams {
            limit: query.limit,
            slug,
            project_id: None,
            srid: Some(query.crs.as_srid()),
        }
    }
}

impl SelectOneParams {
    pub fn from_query(query: &ogc::features::Query) -> Self {
        SelectOneParams {
            project_id: None,
            srid: Some(query.crs.as_srid()),
        }
    }
}
#[derive(Deserialize)]
struct ProjectFeatureRow {
    pub id: i32,
    pub project_id: i32,
    pub collection_id: i32,
    pub properties: serde_json::Value,
    pub name: String,
    pub geometry: Json<geojson::Geometry>,
    pub is_primary: bool,
    pub storage_crs_srid: i32,
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
            collection_id,
            project_id,
            storage_crs_srid,
        } = self;
        let properties = match properties {
            Value::Object(map) => map,
            _ => serde_json::Map::default(),
        };
        Ok(ProjectFeature {
            id,
            properties: Properties {
                collection_id,
                project_id,
                name,
                storage_crs_srid,
                is_primary,
            },
            geometry: geometry.0,
            properties_map: properties,
        })
    }
}

impl SelectOneWithParams for ProjectFeature {
    type Params<'a> = &'a SelectOneParams;

    type Id<'a> = &'a FeatureIdWithCollectionSlug;

    async fn select_one_with_params<'a, 'e, E>(
        executor: E,
        feature_id: Self::Id<'a>,
        params: Self::Params<'a>,
    ) -> Result<Option<Self>, RepositoryError>
    where
        Self: Sized,
        E: sqlx::PgExecutor<'e>,
    {
        let FeatureIdWithCollectionSlug {
            collection_slug,
            id,
        } = feature_id;
        sqlx::query_as!(
            ProjectFeatureRow,
            r#"
            SELECT f.id,
                f.name,
                f.collection_id,
                f.project_id,
                f.is_primary,
                ST_AsGeoJSON(ST_Transform(fo.geom, $3))::jsonb as "geometry!: Json<Geometry>",
                ST_SRID(geom) AS "storage_crs_srid!",
                f.properties
            FROM app.project_features f
            JOIN app.feature_objects fo ON fo.project_feature_id = f.id
            JOIN app.collections c ON f.collection_id = c.id
            WHERE f.id = $1
            AND c.slug = $2
            AND ($4::int IS NULL OR f.project_id = $4)
            "#,
            id,
            collection_slug,
            params.srid.unwrap_or(4326) as i32,
            params.project_id.map(|id| id.0)
        )
        .fetch_optional(executor)
        .await?
        .map(|row| row.try_into())
        .transpose()
    }
}

impl SelectAllWithParamsStreaming for ProjectFeature {
    type Params = SelectAllParams;

    fn select_all_with_params_streaming(
        executor: PoolWrapper,
        params: Self::Params,
    ) -> impl Stream<Item = Result<Self, RepositoryError>> + use<> {
        let SelectAllParams {
            limit,
            slug,
            project_id,
            srid,
        } = params;
        sqlx::query_as!(
            ProjectFeatureRow,
            r#"
            SELECT 
                f.id,
                f.collection_id,
                f.project_id,
                ST_AsGeoJSON(ST_Transform(fo.geom, $1))::jsonb as "geometry!: Json<Geometry>",
                ST_SRID(geom) AS "storage_crs_srid!",
                f.is_primary,
                f.name,
                f.properties 
            FROM app.project_features f
            JOIN app.collections c ON c.id = f.collection_id
            JOIN app.feature_objects fo ON fo.project_feature_id = f.id
            WHERE c.slug = $2
            AND status = 'ACTIVE'
            AND ($3::int IS NULL OR f.project_id = $3)
            ORDER BY f.id
            LIMIT $4
            "#,
            srid.unwrap_or(4326) as i32,
            slug,
            project_id.map(|id| id.0),
            limit.map(|l| l as i64),
        )
        .fetch(executor)
        .map(|res| res?.try_into())
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
            storage_crs_srid: 4626,
            project_id: 0,
            collection_id: 0,
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
