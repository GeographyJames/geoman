use crate::repo::{
    RepositoryError,
    metadata::NumberMatched,
    project::{SelectAllParams, SelectOneParams},
    traits::{SelectAllWithParams, SelectOne, SelectOneWithParams},
};
use domain::{
    Project, ProjectId,
    project::{ProjectName, Properties},
};
use sqlx::types::Json;

#[derive(Debug)]
pub struct ProjectRow {
    id: i32,
    name: String,
    number_matched: i64,
    centroid_in_storage_crs: Option<Json<geojson::Geometry>>,
    geom: Option<Json<geojson::Geometry>>,
}

#[allow(clippy::from_over_into)]
impl Into<Project> for ProjectRow {
    fn into(self) -> Project {
        let ProjectRow {
            id,
            name,
            geom,
            number_matched: _,
            centroid_in_storage_crs,
        } = self;
        let properties = Properties { name };
        Project {
            id,
            properties,
            centroid: geom.map(|g| g.0),
            centroid_in_storage_crs: centroid_in_storage_crs.map(|g| g.0),
        }
    }
}

impl SelectAllWithParams for Project {
    type Params<'a> = SelectAllParams<'a>;
    type MetaData<'a> = NumberMatched;
    async fn select_all_with_params<'a, 'e, E>(
        executor: &'e E,
        params: Self::Params<'a>,
    ) -> Result<(Vec<Self>, Self::MetaData<'a>), RepositoryError>
    where
        &'e E: sqlx::PgExecutor<'e>,
    {
        let SelectAllParams {
            limit,
            crs,
            _bbox: _,
            _bbox_crs: _,
        } = params;
        let rows = sqlx::query_as!(
            ProjectRow,
            r#" WITH primary_boundary_centroid AS (
                              SELECT pf.project_id, ST_Centroid(fo.geom) AS centroid
                                FROM app.project_features pf
                                JOIN app.feature_objects fo
                                        ON fo.collection_id = pf.collection_id
                                       AND fo.project_feature_id = pf.id
                                JOIN app.collections c
                                        ON c.id = pf.collection_id
                               WHERE pf.is_primary=true
                                 AND c.title = 'site boundaries'
                                 )
                SELECT id,
                       name,
                       ST_AsGeoJson(ST_Transform(pb.centroid, $1))::json AS "geom: Json<geojson::Geometry>",
                       ST_AsGeoJson(pb.centroid)::json AS "centroid_in_storage_crs: Json<geojson::Geometry>",
                       COUNT(*) OVER() as "number_matched!"
                  FROM app.projects p
             LEFT JOIN primary_boundary_centroid pb
                        ON pb.project_id = p.id
                 WHERE p.status = 'ACTIVE'
              ORDER BY id
                 LIMIT $2"#,
                 crs.as_srid() as i32,
            limit.map(|l| l as i64)
        )
        .fetch_all(executor)
        .await?;
        let number_matched = rows.first().map(|item| item.number_matched).unwrap_or(0);

        let items = rows
            .into_iter()
            .map(|row| row.into())
            .collect::<Vec<Project>>();
        Ok((items, NumberMatched(number_matched)))
    }
}

impl SelectOne for ProjectName {
    type Id<'a> = ProjectId;
    async fn select_one<'a, 'e, E>(
        executor: &'e E,
        id: Self::Id<'a>,
    ) -> Result<Option<Self>, RepositoryError>
    where
        &'e E: sqlx::PgExecutor<'e>,
    {
        let project_row =
            sqlx::query_scalar!(r#"SELECT name FROM app.projects WHERE id = $1"#, id.0)
                .fetch_optional(executor)
                .await?;

        Ok(project_row.map(ProjectName))
    }
}

impl SelectOneWithParams for Project {
    type Params<'a> = &'a SelectOneParams<'a>;
    type Id<'a> = ProjectId;
    async fn select_one_with_params<'a, 'e, E>(
        executor: &'e E,
        id: Self::Id<'a>,
        params: Self::Params<'a>,
    ) -> Result<Option<Self>, RepositoryError>
    where
        &'e E: sqlx::PgExecutor<'e>,
    {
        let SelectOneParams { crs } = params;
        let project_row = sqlx::query_as!(
            ProjectRow,
               r#" WITH primary_boundary_centroid AS (
                              SELECT pf.project_id, ST_Centroid(fo.geom) AS centroid
                                FROM app.project_features pf
                                JOIN app.feature_objects fo
                                        ON fo.collection_id = pf.collection_id
                                       AND fo.project_feature_id = pf.id
                                JOIN app.collections c
                                        ON c.id = pf.collection_id
                               WHERE pf.is_primary=true
                                 AND c.title = 'site boundaries'
                                 )
                SELECT id,
                       name,
                       ST_AsGeoJson(ST_Transform(pb.centroid, $1))::json AS "geom: Json<geojson::Geometry>",
                       ST_AsGeoJson(pb.centroid)::json AS "centroid_in_storage_crs: Json<geojson::Geometry>",
                       COUNT(*) OVER() as "number_matched!"
                  FROM app.projects p
             LEFT JOIN primary_boundary_centroid pb
                        ON pb.project_id = p.id
                 WHERE p.id = $2"#,
                 crs.as_srid() as i32,
            id.0
        )
        .fetch_optional(executor)
        .await?;

        Ok(project_row.map(|row| row.into()))
    }
}
