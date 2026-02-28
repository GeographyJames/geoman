use chrono::{DateTime, Utc};
use domain::{
    AddedBy, FeatureId, LastUpdatedBy, TurbineLayout, enums::Status,
    turbine_layout::TurbineLayoutProperties,
};
use futures::{Stream, StreamExt};
use geojson::Geometry;
use sqlx::types::Json;

use crate::repo::{
    PoolWrapper, RepositoryError, StreamItem,
    project_features::SelectOneParams,
    traits::{SelectAllWithParamsStreaming, SelectOneWithParams},
    turbine_layout_features::SelectAllParams,
};

struct TurbineLayoutRow {
    pub id: i32,
    pub project_id: i32,
    pub collection_id: i32,
    pub collection_title: String,
    pub name: String,
    pub is_primary: bool,
    pub storage_crs_srid: Option<i32>,
    pub status: Status,
    pub added: DateTime<Utc>,
    pub added_by: AddedBy,
    pub last_updated: DateTime<Utc>,
    pub last_updated_by: LastUpdatedBy,
    pub geometry: Json<Geometry>,
    pub number_matched: i64,
    pub rotor_diameter_mm: Option<i32>,
    pub hub_height_mm: Option<i32>,
    pub turbine_count: i64,
}

impl TryInto<TurbineLayout> for TurbineLayoutRow {
    type Error = RepositoryError;
    fn try_into(self) -> Result<TurbineLayout, RepositoryError> {
        let Self {
            id,
            project_id,
            collection_id,
            collection_title,
            name,
            is_primary,
            storage_crs_srid,
            status,
            added,
            added_by,
            last_updated,
            last_updated_by,
            geometry,
            rotor_diameter_mm,
            hub_height_mm,
            turbine_count,
            ..
        } = self;
        Ok(TurbineLayout {
            id,
            properties_map: serde_json::Map::default(),
            properties: TurbineLayoutProperties {
                id,
                project_id,
                collection_id,
                collection_title,
                name,
                is_primary,
                storage_crs_srid: storage_crs_srid.unwrap_or(0),
                status,
                added,
                added_by,
                last_updated,
                last_updated_by,
                rotor_diameter_mm,
                hub_height_mm,
                turbine_count,
            },
            geometry: geometry.0,
        })
    }
}

impl SelectAllWithParamsStreaming for TurbineLayout {
    type Params<'a> = SelectAllParams;

    fn select_all_with_params_streaming<'a>(
        executor: PoolWrapper,
        params: Self::Params<'a>,
    ) -> impl Stream<Item = Result<StreamItem<Self>, RepositoryError>> + use<> {
        let SelectAllParams {
            project_id,
            crs,
            limit,
            offset,
            status,
        } = params;

        sqlx::query_as!(
            TurbineLayoutRow,
            r#"
            SELECT
                tl.id,
                tl.project_id,
                c.id AS collection_id,
                c.title AS "collection_title!",
                tl.name,
                tl.is_primary,
                tl.status AS "status: Status",
                CASE WHEN COUNT(DISTINCT ST_SRID(t.geom)) = 1
                     THEN MIN(ST_SRID(t.geom))::int
                     ELSE NULL
                END AS storage_crs_srid,
                tl.added,
                ROW(ab.id, ab.first_name, ab.last_name, ab.clerk_id, (ROW(t_ab.id, t_ab.name, t_ab.business_unit_id)::app.team))::app.user AS "added_by!: AddedBy",
                tl.last_updated,
                ROW(ub.id, ub.first_name, ub.last_name, ub.clerk_id, (ROW(t_ub.id, t_ub.name, t_ub.business_unit_id)::app.team))::app.user AS "last_updated_by!: LastUpdatedBy",
                ST_AsGeoJSON(ST_Transform(ST_Collect(t.geom), $1))::jsonb AS "geometry!: Json<Geometry>",
                COUNT(*) OVER () AS "number_matched!",
                CASE WHEN COUNT(DISTINCT t.rotor_diameter_mm) = 1
                     THEN MIN(t.rotor_diameter_mm)::int
                     ELSE NULL
                END AS rotor_diameter_mm,
                CASE WHEN COUNT(DISTINCT t.hub_height_mm) = 1
                     THEN MIN(t.hub_height_mm)::int
                     ELSE NULL
                END AS hub_height_mm,
                COUNT(t.id) AS "turbine_count!"
            FROM app.turbine_layouts tl
            JOIN app.collections c ON c.id = -1
            JOIN app.turbines t ON t.layout_id = tl.id
            JOIN app.users ab ON ab.id = tl.added_by
            JOIN app.teams t_ab ON t_ab.id = ab.team_id
            JOIN app.users ub ON ub.id = tl.last_updated_by
            JOIN app.teams t_ub ON t_ub.id = ub.team_id
            WHERE tl.project_id = $2
              AND tl.status = ANY($3)
            GROUP BY tl.id, c.id, ab.id, t_ab.id, ub.id, t_ub.id
            ORDER BY tl.id
            LIMIT $4
            OFFSET $5
            "#,
            crs.as_srid() as i32,
            project_id.0,
            status.unwrap_or(vec![Status::Active]) as Vec<Status>,
            limit.map(|l| l as i64),
            offset.unwrap_or(0) as i32,
        )
        .fetch(executor)
        .map(|res| {
            let row = res?;
            let number_matched = row.number_matched;
            let item: TurbineLayout = row.try_into()?;
            Ok(StreamItem {
                item,
                number_matched,
            })
        })
    }
}

impl SelectOneWithParams<FeatureId> for TurbineLayout {
    type Params<'a> = &'a SelectOneParams<'a>;

    async fn select_one_with_params<'a, E>(
        executor: &'a E,
        feature_id: FeatureId,
        params: Self::Params<'a>,
    ) -> Result<Option<Self>, RepositoryError>
    where
        &'a E: sqlx::PgExecutor<'a>,
    {
        let row_opt = sqlx::query_as!(
            TurbineLayoutRow,
            r#"
            SELECT
                tl.id,
                tl.project_id,
                c.id AS collection_id,
                c.title AS "collection_title!",
                tl.name,
                tl.is_primary,
                tl.status AS "status: Status",
                CASE WHEN COUNT(DISTINCT ST_SRID(t.geom)) = 1
                     THEN MIN(ST_SRID(t.geom))::int
                     ELSE NULL
                END AS storage_crs_srid,
                tl.added,
                ROW(ab.id, ab.first_name, ab.last_name, ab.clerk_id, (ROW(t_ab.id, t_ab.name, t_ab.business_unit_id)::app.team))::app.user AS "added_by!: AddedBy",
                tl.last_updated,
                ROW(ub.id, ub.first_name, ub.last_name, ub.clerk_id, (ROW(t_ub.id, t_ub.name, t_ub.business_unit_id)::app.team))::app.user AS "last_updated_by!: LastUpdatedBy",
                ST_AsGeoJSON(ST_Transform(ST_Collect(t.geom), $1))::jsonb AS "geometry!: Json<Geometry>",
                1::bigint AS "number_matched!",
                CASE WHEN COUNT(DISTINCT t.rotor_diameter_mm) = 1
                     THEN MIN(t.rotor_diameter_mm)::int
                     ELSE NULL
                END AS rotor_diameter_mm,
                CASE WHEN COUNT(DISTINCT t.hub_height_mm) = 1
                     THEN MIN(t.hub_height_mm)::int
                     ELSE NULL
                END AS hub_height_mm,
                COUNT(t.id) AS "turbine_count!"
            FROM app.turbine_layouts tl
            JOIN app.collections c ON c.id = -1
            JOIN app.turbines t ON t.layout_id = tl.id
            JOIN app.users ab ON ab.id = tl.added_by
            JOIN app.teams t_ab ON t_ab.id = ab.team_id
            JOIN app.users ub ON ub.id = tl.last_updated_by
            JOIN app.teams t_ub ON t_ub.id = ub.team_id
            WHERE tl.id = $2
              AND tl.project_id = $3
            GROUP BY tl.id, c.id, ab.id, t_ab.id, ub.id, t_ub.id
            "#,
            params.crs.as_srid() as i32,
            feature_id.0,
            params.project_id.0,
        )
        .fetch_optional(executor)
        .await?;

        row_opt.map(|r: TurbineLayoutRow| r.try_into()).transpose()
    }
}
