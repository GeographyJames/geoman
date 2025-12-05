use crate::{
    constants::SITE_BOUNDARIES_COLLECTION_NAME,
    postgres::sql_fragments::{user_join_fragment, user_row_fragment},
    repo::{
        RepositoryError,
        project::{SelectAllParams, SelectOneParams},
        traits::{SelectAllWithParams, SelectOne, SelectOneWithParams},
    },
};

use domain::{
    ProjectId,
    project::{Project, ProjectName, ProjectSlugInputDto, Properties},
};
use sqlx::{prelude::FromRow, types::Json};

#[derive(FromRow)]
pub struct ProjectRow {
    id: ProjectId,
    #[sqlx(flatten)]
    properties: Properties,
    centroid_in_storage_crs: Option<Json<geojson::Geometry>>,
    geom: Option<Json<geojson::Geometry>>,
}

#[allow(clippy::from_over_into)]
impl TryInto<Project> for ProjectRow {
    type Error = RepositoryError;
    fn try_into(self) -> Result<Project, RepositoryError> {
        let ProjectRow {
            id,
            properties,
            geom,
            centroid_in_storage_crs,
        } = self;

        Ok(Project {
            id,
            properties,
            centroid: geom.map(|g| g.0),
            centroid_in_storage_crs: centroid_in_storage_crs.map(|g| g.0),
        })
    }
}

fn project_query() -> String {
    format!(
        r#"WITH primary_boundary_centroid AS (
            SELECT pf.project_id, ST_Centroid(fo.geom) AS centroid
              FROM app.project_features pf
              JOIN app.feature_objects fo
                ON fo.collection_id = pf.collection_id
               AND fo.project_feature_id = pf.id
              JOIN app.collections c
                ON c.id = pf.collection_id
             WHERE pf.is_primary = true
               AND c.title = '{SITE_BOUNDARIES_COLLECTION_NAME}'
        )
        SELECT
            p.id,
            p.name,
            p.added,
            p.country_code,
            p.status,
            p.crs_srid,
            p.last_updated,
            p.visibility,
            p.slug,
            p.search_area_id,
            p.search_site_name,
            {user_row_owner},
            {user_row_added_by},
            {user_row_last_updated_by},
            COALESCE(subdivisions.subdivisions, ARRAY[]::app.subdivision[]) AS subdivisions,
            COALESCE(technologies.technologies, ARRAY[]::app.technology[]) AS technologies,
            ST_AsGeoJson(ST_Transform(pb.centroid, $1))::json AS geom,
            ST_AsGeoJson(pb.centroid)::json AS centroid_in_storage_crs
        FROM app.projects p
        {user_join_owner}
        {user_join_added_by}
        {user_join_last_updated_by}
        LEFT JOIN primary_boundary_centroid pb ON pb.project_id = p.id
        LEFT JOIN LATERAL (
            SELECT array_agg(ROW(s.id, s.country_code, s.subdivision_code, s.name)::app.subdivision) AS subdivisions
              FROM app.project_subdivisions ps
              JOIN app.subdivisions s ON s.id = ps.subdivision_id
             WHERE ps.project_id = p.id
        ) subdivisions ON true
        LEFT JOIN LATERAL (
            SELECT array_agg(ROW(t.id, t.name)::app.technology) AS technologies
              FROM app.project_technologies pt
              JOIN app.technologies t ON t.id = pt.technology_id
             WHERE pt.project_id = p.id
        ) technologies ON true
        WHERE p.status = 'ACTIVE'"#,
        user_row_owner = user_row_fragment("o", "owner"),
        user_row_added_by = user_row_fragment("a", "added_by"),
        user_row_last_updated_by = user_row_fragment("l", "last_updated_by"),
        user_join_owner = user_join_fragment("o", "owner"),
        user_join_added_by = user_join_fragment("a", "added_by"),
        user_join_last_updated_by = user_join_fragment("l", "last_updated_by"),
    )
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
        let rows: Vec<ProjectRow> = sqlx::query_as(&format!(
            "{} ORDER BY id
                 LIMIT $2",
            project_query()
        ))
        .bind(crs.as_srid())
        .bind(limit.map(|l| l as i32))
        .fetch_all(executor)
        .await?;

        let items = rows
            .into_iter()
            .map(|row| row.try_into())
            .collect::<Result<Vec<Project>, RepositoryError>>()?;
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

impl SelectOneWithParams<ProjectId> for Project {
    type Params<'a> = &'a SelectOneParams<'a>;

    async fn select_one_with_params<'a, E>(
        executor: &'a E,
        id: ProjectId,
        params: Self::Params<'a>,
    ) -> Result<Option<Self>, RepositoryError>
    where
        &'a E: sqlx::PgExecutor<'a>,
    {
        let SelectOneParams { crs } = params;
        let project_row: Option<ProjectRow> =
            sqlx::query_as(&format!("{} AND p.id = $2", project_query()))
                .bind(crs.as_srid())
                .bind(id.0)
                .fetch_optional(executor)
                .await?;

        project_row.map(|row| row.try_into()).transpose()
    }
}

impl SelectOne<&ProjectSlugInputDto> for ProjectName {
    async fn select_one<'a, E>(
        executor: &'a E,
        id: &ProjectSlugInputDto,
    ) -> Result<Option<Self>, RepositoryError>
    where
        Self: Sized,
        &'a E: sqlx::PgExecutor<'a>,
    {
        sqlx::query_scalar!(
            r#"SELECT name as "name: ProjectName" FROM app.projects WHERE slug = $1"#,
            id.as_ref()
        )
        .fetch_optional(executor)
        .await
        .map_err(Into::into)
    }
}
