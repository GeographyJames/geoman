use crate::repo::{
    RepositoryError,
    project::{SelectAllParams, SelectOneParams},
    traits::{SelectAllWithParams, SelectOne, SelectOneWithParams},
};
use anyhow::Context;
use chrono::{DateTime, Utc};
use domain::{
    Project, ProjectId, Subdivision, Technology, UserId,
    enums::{Status, Visibility},
    project::{ProjectName, Properties},
};

use sqlx::types::Json;

#[derive(Debug)]
pub struct ProjectRow {
    id: ProjectId,
    name: String,
    added: DateTime<Utc>,
    owner: UserId,
    added_by: UserId,
    technologies: Option<Json<Vec<Technology>>>,
    country_code: String,
    subdivisions: Option<Json<Vec<Subdivision>>>,
    status: Status,
    visibility: Visibility,
    last_updated_by: UserId,
    last_updated: DateTime<Utc>,
    centroid_in_storage_crs: Option<Json<geojson::Geometry>>,
    geom: Option<Json<geojson::Geometry>>,
    crs_srid: Option<i32>,
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
            owner,
            added_by,
            technologies: technologies.map(|t| t.0).unwrap_or_default(),
            country: isocountry::CountryCode::for_alpha2(&country_code)
                .context("failed to parse country code")?,
            subdivisions: subdivisions.map(|s| s.0).unwrap_or_default(),
            status,
            visibility,
            crs_srid,
            last_updated_by,
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
                SELECT id AS "id: ProjectId",
                       name,
                       added,
                       owner AS "owner: UserId",
                       added_by AS "added_by: UserId",
                       country_code,
                       status AS "status: Status",
                       crs_srid,
                       last_updated,
                       visibility AS "visibility: Visibility",
                       sub1.subdivisions AS "subdivisions: Json<Vec<Subdivision>>",
                       sub2.technologies AS "technologies: Json<Vec<Technology>>",
                       last_updated_by AS "last_updated_by: UserId",
                       ST_AsGeoJson(ST_Transform(pb.centroid, $1))::json AS "geom: Json<geojson::Geometry>",
                       ST_AsGeoJson(pb.centroid)::json AS "centroid_in_storage_crs: Json<geojson::Geometry>"
                  FROM app.projects p
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
                         ) sub1 ON true

    LEFT JOIN LATERAL (SELECT json_agg(json_build_object(
                            'id', t.id,
                            'name', t.name
                            )) AS technologies
                        FROM app.project_technologies pt
                        JOIN app.technologies t ON t.id = pt.technology_id
                        WHERE pt.project_id = p.id
                        ) sub2 ON true

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
                SELECT id AS "id: ProjectId",
                       name,
                       added,
                       owner AS "owner: UserId",
                       added_by AS "added_by: UserId",
                       country_code,
                       status AS "status: Status",
                       crs_srid,
                       last_updated,
                       visibility AS "visibility: Visibility",
                       sub1.subdivisions AS "subdivisions: Json<Vec<Subdivision>>",
                       sub2.technologies AS "technologies: Json<Vec<Technology>>",
                       last_updated_by AS "last_updated_by: UserId",
                       ST_AsGeoJson(ST_Transform(pb.centroid, $1))::json AS "geom: Json<geojson::Geometry>",
                       ST_AsGeoJson(pb.centroid)::json AS "centroid_in_storage_crs: Json<geojson::Geometry>"
                  FROM app.projects p
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
                         ) sub1 ON true

    LEFT JOIN LATERAL (SELECT json_agg(json_build_object(
                            'id', t.id,
                            'name', t.name
                            )) AS technologies
                        FROM app.project_technologies pt
                        JOIN app.technologies t ON t.id = pt.technology_id
                        WHERE pt.project_id = p.id
                        ) sub2 ON true
                 WHERE p.id = $2"#,
                 crs.as_srid() as i32,
            id.0
        )
        .fetch_optional(executor)
        .await?;

        project_row.map(|row| row.try_into()).transpose()
    }
}
