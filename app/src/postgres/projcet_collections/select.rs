use domain::{
    CollectionListItem, ProjectCollection, ProjectCollectionId, SupportedCrs,
    enums::{CollectionId, GeometryType},
};
use ogcapi_types::common::{Bbox, Crs, SpatialExtent};

use crate::repo::{
    RepositoryError,
    project_collections::{SelectAllParams, SelectOneParams},
    traits::{SelectAll, SelectAllWithParams, SelectOneWithParams},
};

struct CollectionRow {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub storage_crs_srid: Option<i32>,
    pub extent: Option<Vec<f64>>,
    pub geometry_type: GeometryType,
}

impl CollectionRow {
    fn into_collection(self, extent_crs: Crs) -> ProjectCollection {
        let Self {
            id,
            title,
            description,
            storage_crs_srid,
            extent,
            geometry_type,
        } = self;
        let bbox: Option<Bbox> = extent.and_then(|bbox| Bbox::try_from(bbox.as_slice()).ok());
        let storage_crs = storage_crs_srid.map(Crs::from_srid);
        let supported_crs = SupportedCrs::new(storage_crs.clone());

        ProjectCollection {
            id: CollectionId::ProjectCollection(ProjectCollectionId(id)),
            title,
            description,
            supported_crs,
            storage_crs,
            geometry_type,
            extent: bbox.map(|bbox| SpatialExtent {
                bbox: vec![bbox],
                crs: extent_crs,
            }),
        }
    }
}

impl SelectOneWithParams<ProjectCollectionId> for ProjectCollection {
    type Params<'a> = &'a SelectOneParams;
    async fn select_one_with_params<'a, E>(
        executor: &'a E,
        id: ProjectCollectionId,
        params: Self::Params<'a>,
    ) -> Result<Option<Self>, RepositoryError>
    where
        &'a E: sqlx::PgExecutor<'a>,
    {
        let extent_crs = Crs::default();
        let row_opt = sqlx::query_as!(
            CollectionRow,
            r#"
            SELECT id,
                   title,
                   description,
                   geometry_type as "geometry_type: GeometryType",
                   (SELECT CASE WHEN COUNT(DISTINCT ST_SRID(fo.geom)) = 1
                           THEN MIN(ST_SRID(fo.geom))::int
                           ELSE NULL
                       END
                      FROM app.project_features f
                      JOIN app.feature_objects fo ON fo.project_feature_id = f.id
                     WHERE f.collection_id = c.id
                     AND f.project_id = $1
                      ) as storage_crs_srid,
                        (SELECT CASE
                                    WHEN bbox IS NOT NULL THEN
                 ARRAY[
                     ST_XMin(bbox),
                     ST_YMin(bbox),
                     ST_XMax(bbox),
                     ST_YMax(bbox)
                 ]
             ELSE NULL
         END
         FROM (
             SELECT ST_Extent(ST_Transform(fo.geom, $3))::geometry as bbox
             FROM app.project_features f
             JOIN app.feature_objects fo ON fo.project_feature_id = f.id
             WHERE f.collection_id = c.id
             AND f.project_id = $1
         ) extent_sub) as extent


              FROM app.collections c
  WHERE EXISTS (
      SELECT 1
      FROM app.project_features f
      WHERE f.collection_id = c.id
      AND f.project_id =  $1
  )
  AND c.id = $2"#,
            params.project_id.0,
            id.0,
            extent_crs.as_srid() as i32
        )
        .fetch_optional(executor)
        .await?;

        match row_opt {
            Some(row) => Ok(Some(row.into_collection(extent_crs))),
            None => Ok(None),
        }
    }
}

impl SelectAllWithParams for ProjectCollection {
    type Params<'a> = &'a SelectAllParams;
    type MetaData<'a> = ();
    async fn select_all_with_params<'a, E>(
        executor: &'a E,
        params: Self::Params<'a>,
    ) -> Result<(Vec<Self>, ()), RepositoryError>
    where
        &'a E: sqlx::PgExecutor<'a>,
    {
        let extent_crs = Crs::default();

        let rows = sqlx::query_as!(
            CollectionRow,
            r#"
            SELECT id,
                   title,
                   description,
                   geometry_type AS "geometry_type: GeometryType",
                   (SELECT CASE WHEN COUNT(DISTINCT ST_SRID(fo.geom)) = 1
                           THEN MIN(ST_SRID(fo.geom))::int
                           ELSE NULL
                       END
                      FROM app.project_features f
                      JOIN app.feature_objects fo ON fo.project_feature_id = f.id
                     WHERE f.collection_id = c.id
                       AND f.project_id = $1
                ) as storage_crs_srid,
                        (SELECT CASE
                                    WHEN bbox IS NOT NULL THEN
                 ARRAY[
                     ST_XMin(bbox),
                     ST_YMin(bbox),
                     ST_XMax(bbox),
                     ST_YMax(bbox)
                 ]
             ELSE NULL
         END
         FROM (
             SELECT ST_Extent(ST_Transform(fo.geom, $2))::geometry as bbox
             FROM app.project_features f
             JOIN app.feature_objects fo ON fo.project_feature_id = f.id
             WHERE f.collection_id = c.id
             AND f.project_id = $1
         ) extent_sub) as extent
  FROM app.collections c
  WHERE EXISTS (
      SELECT 1
      FROM app.project_features f
      WHERE f.collection_id = c.id
      AND f.project_id =  $1
  )
  ORDER BY id"#,
            params.project_id.0,
            extent_crs.as_srid() as i32
        )
        .fetch_all(executor)
        .await?;

        let mut items = Vec::new();
        for row in rows {
            items.push(row.into_collection(extent_crs.clone()));
        }
        Ok((items, ()))
    }
}

impl SelectAll for CollectionListItem {
    async fn select_all<'a, E>(executor: &'a E) -> Result<Vec<Self>, RepositoryError>
    where
        Self: Sized,
        &'a E: sqlx::PgExecutor<'a>,
    {
        sqlx::query_as!(
            CollectionListItem,
            r#"SELECT id AS "id: ProjectCollectionId",
                      title,
                      description,
                      geometry_type AS "geometry_type: GeometryType"
               FROM app.collections
               ORDER BY id"#
        )
        .fetch_all(executor)
        .await
        .map_err(Into::into)
    }
}
