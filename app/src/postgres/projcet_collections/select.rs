use domain::{
    AddedBy, CollectionListItem, ProjectCollection, ProjectCollectionId, SupportedCrs,
    enums::{CollectionId, GeometryType, Status},
};
use ogcapi_types::common::{Bbox, Crs, SpatialExtent};

use crate::repo::{
    RepositoryError,
    project_collections::{SelectAllParams, SelectOneParams},
    traits::{SelectAll, SelectAllWithParams, SelectOneWithParams},
};

struct CollectionRow {
    pub id: i32,
    pub slug: String,
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
            slug,
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
            slug,
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
                   slug,
                   description,
                   geometry_type as "geometry_type: GeometryType",
                   (SELECT CASE WHEN COUNT(DISTINCT ST_SRID(f.geom)) = 1
                           THEN MIN(ST_SRID(f.geom))::int
                           ELSE NULL
                       END
                      FROM app.project_features f
                     WHERE f.collection_id = c.id
                       AND f.project_id = $1
                       AND f.status = ANY($4)
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
                        SELECT ST_Extent(ST_Transform(f.geom, $3))::geometry as bbox
                        FROM app.project_features f
                        WHERE f.collection_id = c.id
                          AND f.project_id = $1
                          AND f.status = ANY($4)
                    ) extent_sub) as extent
              FROM app.collections c
             WHERE EXISTS (
                 SELECT 1
                 FROM app.project_features f
                 WHERE f.collection_id = c.id
                   AND f.project_id = $1
                   AND f.status = ANY($4)

             )
               AND c.id = $2"#,
            params.project_id.0,
            id.0,
            extent_crs.as_srid() as i32,
            params.status.clone().unwrap_or(vec![Status::Active]) as Vec<Status>
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
                   slug,
                   description,
                   geometry_type AS "geometry_type: GeometryType",
                   (SELECT CASE WHEN COUNT(DISTINCT ST_SRID(f.geom)) = 1
                           THEN MIN(ST_SRID(f.geom))::int
                           ELSE NULL
                       END
                      FROM app.project_features f
                     WHERE f.collection_id = c.id
                       AND f.project_id = $1
                       AND f.status = ANY($3)
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
                        SELECT ST_Extent(ST_Transform(f.geom, $2))::geometry as bbox
                        FROM app.project_features f
                        WHERE f.collection_id = c.id
                          AND f.project_id = $1
                          AND f.status = ANY($3)
                    ) extent_sub) as extent
  FROM app.collections c
  WHERE c.status = 'ACTIVE'
  AND EXISTS (
      SELECT 1
      FROM app.project_features f
      WHERE f.collection_id = c.id
      AND f.status = ANY($3)
      AND f.project_id =  $1
  )
  ORDER BY id"#,
            params.project_id.0,
            extent_crs.as_srid() as i32,
            params.status.clone().unwrap_or(vec![Status::Active]) as Vec<Status>
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
            r#"SELECT c.id AS "id: ProjectCollectionId",
                      c.title,
                      c.description,
                      c.geometry_type AS "geometry_type: GeometryType",
                      COUNT(f.id) FILTER (WHERE f.status = 'ACTIVE') AS "active_feature_count!",
                      COUNT(f.id) FILTER (WHERE f.status = 'ARCHIVED') AS "archived_feature_count!",
                      c.added,
                      ROW(ab.id, ab.first_name, ab.last_name, ab.clerk_id, (ROW(t.id, t.name, t.business_unit_id)::app.team))::app.user AS "added_by!: AddedBy"
               FROM app.collections c
               LEFT JOIN app.project_features f ON f.collection_id = c.id
               JOIN app.users ab ON ab.id = c.added_by
               JOIN app.teams t ON t.id = ab.team_id
               WHERE c.status = 'ACTIVE'
               GROUP BY c.id, ab.id, t.id
               ORDER BY c.id"#
        )
        .fetch_all(executor)
        .await
        .map_err(Into::into)
    }
}
