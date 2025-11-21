use domain::{Collection, ProjectCollectionId, ProjectId};
use ogcapi_types::common::{Bbox, Crs, SpatialExtent};

use crate::{
    errors::RepositoryError,
    postgres::traits::{SelectAll, SelectAllWithParams, SelectOne, SelectOneWithParams},
};

pub struct SelectOneParams {
    pub project_id: ProjectId,
}

pub struct SelectAllParams {
    pub project_id: ProjectId,
}

pub struct CollectionRow {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub storage_crs_srid: Option<i32>,
    pub extent: Option<Vec<f64>>,
}

impl CollectionRow {
    fn try_into_collection(self, extent_crs: Crs) -> Result<Collection, RepositoryError> {
        let Self {
            id,
            title,
            description,
            storage_crs_srid,
            extent,
        } = self;
        let bbox: Option<Bbox> = extent
            .map(|bbox| Bbox::try_from(bbox.as_slice()))
            .transpose()
            .map_err(|e| {
                RepositoryError::UnexpectedError(anyhow::anyhow!(
                    "collection has invalid bounding box: {}",
                    e
                ))
            })?;

        Ok(Collection {
            id,
            title,
            description,
            storage_crs_srid,
            extent: bbox.map(|bbox| SpatialExtent {
                bbox: vec![bbox],
                crs: extent_crs,
            }),
        })
    }
}

impl SelectAll for Collection {
    async fn select_all<'e, E>(executor: E) -> Result<Vec<Self>, RepositoryError>
    where
        E: sqlx::PgExecutor<'e>,
    {
        let extent_crs = Crs::default();
        sqlx::query_as!(
            CollectionRow,
            "SELECT id,
                    title,
                    description,
                    (SELECT CASE WHEN COUNT(DISTINCT ST_SRID(fo.geom)) = 1
                            THEN MIN(ST_SRID(fo.geom))::int
                            ELSE NULL
                        END
                       FROM app.project_features f
                       JOIN app.feature_objects fo ON fo.project_feature_id = f.id
                      WHERE f.collection_id = c.id
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
             SELECT ST_Extent(ST_Transform(fo.geom, $1))::geometry as bbox
             FROM app.project_features f
             JOIN app.feature_objects fo ON fo.project_feature_id = f.id
             WHERE f.collection_id = c.id
         ) extent_sub) as extent 
                    FROM app.collections c ORDER BY id",
            extent_crs.as_srid() as i32
        )
        .fetch_all(executor)
        .await?
        .into_iter()
        .map(|row| row.try_into_collection(extent_crs.clone()))
        .collect()
    }
}

impl SelectOne for Collection {
    type Id<'a> = ProjectCollectionId;
    async fn select_one<'a, 'e, E>(
        executor: E,
        id: Self::Id<'a>,
    ) -> Result<Option<Self>, RepositoryError>
    where
        E: sqlx::PgExecutor<'e>,
    {
        let extent_crs = Crs::default();
        sqlx::query_as!(
            CollectionRow,
            "
            SELECT id,
                   title,
                   description,
                   (SELECT CASE WHEN COUNT(DISTINCT ST_SRID(fo.geom)) = 1
                           THEN MIN(ST_SRID(fo.geom))::int
                           ELSE NULL
                       END
                      FROM app.project_features f
                      JOIN app.feature_objects fo ON fo.project_feature_id = f.id
                     WHERE f.collection_id = c.id
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
         ) extent_sub) as extent 
              FROM app.collections c WHERE c.id = $1",
            id.0,
            extent_crs.as_srid() as i32
        )
        .fetch_optional(executor)
        .await?
        .map(|row| row.try_into_collection(extent_crs))
        .transpose()
    }
}

impl SelectOneWithParams for Collection {
    type Params<'a> = &'a SelectOneParams;
    type Id<'a> = ProjectCollectionId;
    async fn select_one_with_params<'a, 'e, E>(
        executor: E,
        id: Self::Id<'a>,
        params: Self::Params<'a>,
    ) -> Result<Option<Self>, RepositoryError>
    where
        Self: Sized,
        E: sqlx::PgExecutor<'e>,
    {
        let extent_crs = Crs::default();
        sqlx::query_as!(
            CollectionRow,
            r#"
            SELECT id,
                   title,
                   description,
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
        .await?
        .map(|row| row.try_into_collection(extent_crs))
        .transpose()
    }
}

impl SelectAllWithParams for Collection {
    type Params<'a> = &'a SelectAllParams;
    async fn select_all_with_params<'a, 'e, E>(
        executor: E,
        params: Self::Params<'a>,
    ) -> Result<Vec<Self>, RepositoryError>
    where
        E: sqlx::PgExecutor<'e>,
    {
        let extent_crs = Crs::default();
        sqlx::query_as!(
            CollectionRow,
            r#"
            SELECT id,
                   title,
                   description,
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
        .await?
        .into_iter()
        .map(|row| row.try_into_collection(extent_crs.clone()))
        .collect()
    }
}
