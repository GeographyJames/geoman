use crate::{
    constants::SITE_BOUNDARIES_COLLECTION_NAME,
    repo::{
        RepositoryError,
        project::{SelectAllParams, SelectOneParams},
        traits::{SelectAllWithParams, SelectOne, SelectOneWithParams},
    },
};
use anyhow::Context;
use chrono::{DateTime, Utc};
use domain::{
    Project, ProjectId, Subdivision, Technology, User, UserId,
    enums::{Status, Visibility},
    project::{ProjectName, Properties},
};
use sqlx::{prelude::FromRow, types::Json};

#[derive(Debug, FromRow)]
pub struct ProjectRow {
    id: ProjectId,
    name: String,
    added: DateTime<Utc>,
    owner: User,
    added_by: User,
    technologies: Option<Json<Vec<Technology>>>,
    country_code: String,
    subdivisions: Option<Json<Vec<Subdivision>>>,
    status: Status,
    visibility: Visibility,
    last_updated_by: User,
    last_updated: DateTime<Utc>,
    centroid_in_storage_crs: Option<Json<geojson::Geometry>>,
    geom: Option<Json<geojson::Geometry>>,
    crs_srid: Option<i32>,
}

const ROWS: &str = r#"
p.id,
p.name,
p.added,
p.country_code,
p.status,
p.crs_srid,
p.last_updated,
p.visibility,
subdivisions.subdivisions,
technologies.technologies,
ST_AsGeoJson(ST_Transform(pb.centroid, $1))::json AS geom,
ST_AsGeoJson(pb.centroid)::json AS centroid_in_storage_crs
"#;

fn user_query(user_alias: &str, team_alias: &str) -> String {
    format!(
        r#"ROW({u}.id, {u}.first_name, {u}.last_name, {u}.clerk_id, ROW({t}.id, {t}.name)::app.team )::app.user AS {u}"#,
        u = user_alias,
        t = team_alias
    )
}

#[allow(clippy::from_over_into)]
impl TryInto<Project> for ProjectRow {
    type Error = RepositoryError;
    fn try_into(self) -> Result<Project, RepositoryError> {
        let ProjectRow {
            id,
            name,
            geom,
            centroid_in_storage_crs,
            added,
            owner,
            added_by,
            technologies,
            country_code,
            subdivisions,
            status,
            visibility,
            crs_srid,
            last_updated_by,
            last_updated,
        } = self;
        let properties = Properties {
            name,
            added,
            owner: owner,
            added_by: added_by,
            technologies: technologies.map(|t| t.0).unwrap_or_default(),
            country_code,
            subdivisions: subdivisions.map(|s| s.0).unwrap_or_default(),
            status,
            visibility,
            crs_srid,
            last_updated_by: last_updated_by,
            last_updated,
        };
        Ok(Project {
            id,
            properties,
            centroid: geom.map(|g| g.0),
            centroid_in_storage_crs: centroid_in_storage_crs.map(|g| g.0),
        })
    }
}

fn project_query() -> String {
    let owner_col = "owner";
    let added_by_col = "added_by";
    let last_updated_by_col = "last_updated_by";
    format!(
        r#" WITH primary_boundary_centroid AS (
                              SELECT pf.project_id, ST_Centroid(fo.geom) AS centroid
                                FROM app.project_features pf
                                JOIN app.feature_objects fo
                                        ON fo.collection_id = pf.collection_id
                                       AND fo.project_feature_id = pf.id
                                JOIN app.collections c
                                        ON c.id = pf.collection_id
                               WHERE pf.is_primary=true
                                 AND c.title = '{SITE_BOUNDARIES_COLLECTION_NAME}'
                                 )
                SELECT {ROWS},
                    {}, {}, {}
                  FROM app.projects p
                  JOIN app.users {o} ON {o}.id = p.{o}
                  JOIN app.teams ot ON owner.team_id = ot.id
                  JOIN app.users {ab} ON {ab}.id = p.{ab}
                  JOIN app.teams at ON at.id = added_by.team_id
                  JOIN app.users {lb} ON {lb}.id = p.{lb}
                  JOIN app.teams lt ON lt.id = last_updated_by.id
             LEFT JOIN primary_boundary_centroid pb
                        ON pb.project_id = p.id
    





     LEFT JOIN LATERAL (SELECT json_agg(json_build_object(
                                'id', s.id,
                                'country_code', s.country_code,
                                'subdivision_code', s.subdivision_code,
                                'name', s.name
                               )) AS subdivisions
                          FROM app.project_subdivisions ps
                          JOIN app.subdivisions s ON s.id = ps.subdivision_id
                         WHERE ps.project_id = p.id
                         ) subdivisions ON true

    LEFT JOIN LATERAL (SELECT json_agg(json_build_object(
                            'id', t.id,
                            'name', t.name
                            )) AS technologies
                        FROM app.project_technologies pt
                        JOIN app.technologies t ON t.id = pt.technology_id
                        WHERE pt.project_id = p.id
                        ) technologies ON true

                 WHERE p.status = 'ACTIVE'"#,
        user_query(owner_col, "ot"),
        user_query(added_by_col, "at"),
        user_query(last_updated_by_col, "lt"),
        o = owner_col,
        ab = added_by_col,
        lb = last_updated_by_col,
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
