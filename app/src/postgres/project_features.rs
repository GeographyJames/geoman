use domain::{ProjectFeature, ProjectFeatureId, poject_feature::Properties};
use futures::{Stream, StreamExt};
use geojson::Geometry;
use ogcapi_types::common::Crs;
use serde::Deserialize;
use serde_json::Value;
use sqlx::types::Json;

use crate::repo::{
    PoolWrapper, RepositoryError, StreamItem,
    project_features::{SelectAllParams, SelectOneParams},
    traits::{SelectAllWithParamsStreaming, SelectOneWithParams},
};

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
    pub number_matched: i64,
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
            ..
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

                storage_crs: Crs::from_srid(storage_crs_srid),
                is_primary,
            },
            geometry: geometry.0,
            properties_map: properties,
        })
    }
}

impl SelectOneWithParams for ProjectFeature {
    type Params<'a> = &'a SelectOneParams<'a>;

    type Id<'a> = &'a ProjectFeatureId;

    async fn select_one_with_params<'a, E>(
        executor: &'a E,
        feature_id: Self::Id<'a>,
        params: Self::Params<'a>,
    ) -> Result<Option<Self>, RepositoryError>
    where
        &'a E: sqlx::PgExecutor<'a>,
    {
        let ProjectFeatureId { collection_id, id } = feature_id;
        let SelectOneParams { project_id, crs } = params;

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
                f.properties,
                1 as "number_matched!"
            FROM app.project_features f
            JOIN app.feature_objects fo ON fo.project_feature_id = f.id
            JOIN app.collections c ON f.collection_id = c.id
            WHERE f.id = $1
            AND c.id = $2
            AND ($4::int IS NULL OR f.project_id = $4)
            "#,
            id,
            collection_id.0,
            crs.as_srid() as i32,
            project_id.0
        )
        .fetch_optional(executor)
        .await?
        .map(|row| row.try_into())
        .transpose()
    }
}

impl SelectAllWithParamsStreaming for ProjectFeature {
    type Params<'a> = SelectAllParams;

    fn select_all_with_params_streaming<'a>(
        executor: PoolWrapper,
        params: Self::Params<'a>,
    ) -> impl Stream<Item = Result<StreamItem<Self>, RepositoryError>> + use<> {
        let SelectAllParams {
            limit,
            project_id,
            crs,
            bbox,
            bbox_crs,
            collection_id,
            offset,
            ..
        } = params;
        let bbox = bbox.map(|bbox| match bbox {
            ogcapi_types::common::Bbox::Bbox2D(bbox) => bbox,
            ogcapi_types::common::Bbox::Bbox3D(bbox) => [bbox[0], bbox[1], bbox[3], bbox[4]],
        });

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
                f.properties,
                COUNT(*) OVER() as "number_matched!"

            FROM app.project_features f
            JOIN app.collections c ON c.id = f.collection_id
            JOIN app.feature_objects fo ON fo.project_feature_id = f.id
            WHERE c.id = $2
            AND status = 'ACTIVE'
            AND ($3::int IS NULL OR f.project_id = $3)
            AND ($4::float IS NULL OR (
                fo.geom && ST_Transform(ST_MakeEnvelope($4, $5, $6, $7, $8), ST_SRID(fo.geom))
                ))
            ORDER BY f.id
            LIMIT $9
            OFFSET $10
            "#,
            crs.as_srid() as i32,
            collection_id.0,
            project_id.0,
            bbox.map(|bbox| bbox[0]),
            bbox.map(|bbox| bbox[1]),
            bbox.map(|bbox| bbox[2]),
            bbox.map(|bbox| bbox[3]),
            bbox_crs.unwrap_or_default().as_srid() as i32,
            limit.map(|l| l as i64),
            offset.unwrap_or(0) as i32
        )
        .fetch(executor)
        .map(|res| {
            let row = res?;
            let number_matched = row.number_matched;
            let item: ProjectFeature = row.try_into()?;
            Ok(StreamItem {
                item,
                number_matched,
            })
        })
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
            number_matched: 1,

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
