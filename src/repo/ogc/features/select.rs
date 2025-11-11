use sqlx::types::Json;

use crate::{
    domain::{CollectionId, FeatureId},
    repo::{
        ogc::FeatureRow,
        traits::{SelectOne, SelectOneWithParams},
    },
};

impl SelectOne for Json<FeatureRow> {
    type Id<'a> = &'a FeatureId;
    async fn select_one<'a, 'e, E>(
        executor: E,
        id: Self::Id<'a>,
    ) -> Result<Option<Self>, sqlx::Error>
    where
        E: sqlx::PgExecutor<'e>,
    {
        sqlx::query_scalar!(
            r#"
            SELECT jsonb_build_object(
                'id', id,
                'geometry', ST_AsGeoJSON(ST_Transform(geom, 4326))::jsonb,
                'properties',  properties || jsonb_build_object('name', name, 'is_primary', is_primary) 
            ) as "feature!: Json<FeatureRow>"
            FROM app.features
            WHERE id = $1
            "#,
            id.0
        )
        .fetch_optional(executor)
        .await
    }
}

pub struct DbQueryParams {
    pub limit: Option<i64>,
}

impl SelectOneWithParams for Json<Vec<FeatureRow>> {
    type Id<'a> = &'a CollectionId;
    type Params<'a> = &'a DbQueryParams;
    async fn select_one_with_params<'a, 'e, E>(
        executor: E,
        id: Self::Id<'a>,
        params: Self::Params<'a>,
    ) -> Result<Option<Self>, sqlx::Error>
    where
        E: sqlx::PgExecutor<'e>,
    {
        sqlx::query_scalar!(
            r#"
            SELECT COALESCE(
                json_agg(
                    jsonb_build_object(
                        'id', id,
                        'geometry', ST_AsGeoJSON(ST_Transform(geom, 4326))::jsonb,
                        'properties', properties || jsonb_build_object('name', name, 'is_primary', is_primary)
                    )
                ),
                '[]'::json
            ) as "features!: Json<Vec<FeatureRow>>"
            FROM (
                SELECT id, name, is_primary, properties, geom
                FROM app.features
                WHERE collection_id = $1 AND status = 'ACTIVE'
                ORDER BY id
                LIMIT $2
            ) t
            "#,
            id.0,
            params.limit
        )
        .fetch_optional(executor)
        .await
    }
}
