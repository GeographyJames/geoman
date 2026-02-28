use crate::{
    constants::SITE_BOUNDARIES_COLLECTION_ID,
    postgres::sql_fragments::{user_join_fragment, user_row_fragment},
    repo::{
        RepositoryError,
        project::{SelectAllParams, SelectOneParams},
        traits::{SelectAllWithParams, SelectOne, SelectOneWithParams},
    },
};

use domain::{
    ProjectId,
    enums::Status,
    project::{Project, ProjectName, Properties},
};
use sqlx::{prelude::FromRow, types::Json};

#[derive(FromRow)]
pub struct ProjectRow {
    id: ProjectId,
    #[sqlx(flatten)]
    properties: Properties,
    geom: Option<Json<geojson::Geometry>>,
}

impl TryInto<Project> for ProjectRow {
    type Error = RepositoryError;
    fn try_into(self) -> Result<Project, RepositoryError> {
        let ProjectRow {
            id,
            properties,
            geom,
        } = self;

        Ok(Project {
            id,
            properties,
            centroid: geom.map(|g| g.0),
        })
    }
}

fn project_query() -> String {
    format!(
        r#"WITH primary_boundary_centroid AS (
            SELECT pf.project_id, ST_Centroid(pf.geom) AS centroid
              FROM app.project_features pf
              JOIN app.collections c
                ON c.id = pf.collection_id
             WHERE pf.is_primary = true
               AND c.id = {SITE_BOUNDARIES_COLLECTION_ID}
        )
        SELECT
            p.id,
            p.name,
            p.added,
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
            ST_AsGeoJson(ST_Transform(pb.centroid, $1))::json AS geom,
            (SELECT substring(srtext from '"([^"]+)"') FROM spatial_ref_sys WHERE srid = p.crs_srid) AS crs_name,
            ST_X(ST_Transform(pb.centroid, p.crs_srid)) AS centroid_x,
            ST_Y(ST_Transform(pb.centroid, p.crs_srid)) AS centroid_y
        FROM app.projects p
        {user_join_owner}
        {user_join_added_by}
        {user_join_last_updated_by}
        LEFT JOIN primary_boundary_centroid pb ON pb.project_id = p.id
        "#,
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
            status,
        } = params;
        let rows: Vec<ProjectRow> = sqlx::query_as(&format!(
            "{}         WHERE p.status = ANY($2) AND p.search_area_id IS NULL
            ORDER BY id
                 LIMIT $3",
            project_query()
        ))
        .bind(crs.as_srid())
        .bind(status.unwrap_or(vec![Status::Active]))
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
