use crate::repo::{
    RepositoryError,
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
    type MetaData<'a> = ();
    async fn select_all_with_params<'a, E>(
        executor: &'a E,
        params: Self::Params<'a>,
    ) -> Result<(Vec<Self>, Self::MetaData<'a>), RepositoryError>
    where
        &'a E: sqlx::PgExecutor<'a>,
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
                       ST_AsGeoJson(pb.centroid)::json AS "centroid_in_storage_crs: Json<geojson::Geometry>"
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

        let items = rows
            .into_iter()
            .map(|row| row.into())
            .collect::<Vec<Project>>();
        Ok((items, ()))
    }
}

impl SelectOne<ProjectId> for ProjectName {
    async fn select_one<'a, E>(
        executor: &'a E,
        id: ProjectId,
    ) -> Result<Option<Self>, RepositoryError>
    where
        &'a E: sqlx::PgExecutor<'a>,
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
    async fn select_one_with_params<'a, E>(
        executor: &'a E,
        id: Self::Id<'a>,
        params: Self::Params<'a>,
    ) -> Result<Option<Self>, RepositoryError>
    where
        &'a E: sqlx::PgExecutor<'a>,
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
                       ST_AsGeoJson(pb.centroid)::json AS "centroid_in_storage_crs: Json<geojson::Geometry>"
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
