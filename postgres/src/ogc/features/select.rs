use crate::{
    PoolWrapper,
    traits::{SelectAllWithParamsStreaming, SelectOne},
};
use futures::{Stream, StreamExt};
use ogc::types::features::{FeatureRow, Query};
use sqlx::types::Json;

impl SelectOne for FeatureRow {
    type Id<'a> = i32;
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
            id
        )
        .fetch_optional(executor)
        .await.map(|opt|opt.map(|json|json.0))
    }
}

#[derive(Clone)]
#[non_exhaustive]
pub struct SelectAllParams {
    pub limit: Option<usize>,
    pub slug: String,
}

impl SelectAllParams {
    pub fn from_query(query: Query, slug: String) -> Self {
        SelectAllParams {
            limit: query.limit,
            slug,
        }
    }
}

impl SelectAllWithParamsStreaming for FeatureRow {
    type Params = SelectAllParams;

    fn select_all_with_params_streaming(
        executor: PoolWrapper,
        params: Self::Params,
    ) -> impl Stream<Item = Result<FeatureRow, sqlx::Error>> + use<> {
        sqlx::query_scalar!(
            r#"
            SELECT jsonb_build_object(
            'id', f.id,
            'geometry', ST_AsGeoJSON(ST_Transform(f.geom, 4326))::jsonb,
            'properties', f.properties ||  jsonb_build_object('name', f.name, 'is_primary', f.is_primary)
        )
            as "feature!: Json<FeatureRow>"
                FROM app.features f
                JOIN app.collections c ON c.id = f.collection_id
                WHERE c.slug = $1 AND status = 'ACTIVE'
                ORDER BY f.id
                LIMIT $2
            "#,
            params.slug,
            params.limit.map(|l| l as i64)
        )
        .fetch(executor)
        .map(|res| res.map(|json| json.0))
    }
}

// impl FeatureRow {
//     pub async fn select_all_features_by_collection<'a, 'e, E>(
//         executor: E,
//         params: &SelectAllParams<'a>,
//     ) -> Result<Option<Vec<FeatureRow>>, sqlx::Error>
//     where
//         E: sqlx::PgExecutor<'e>,
//     {
//         sqlx::query_scalar!(
//             r#"
// SELECT COALESCE(
//           (
//               SELECT json_agg(
//                   jsonb_build_object(
//                       'id', f.id,
//                       'geometry',
//   ST_AsGeoJSON(ST_Transform(f.geom, 4326))::jsonb,
//                       'properties', f.properties ||
//   jsonb_build_object('name', f.name, 'is_primary', f.is_primary)
//                   )
//               )
//               FROM (
//                   SELECT id, name, is_primary, properties, geom
//                   FROM app.features
//                   WHERE collection_id = c.id
//                     AND status = 'ACTIVE'
//                   ORDER BY id
//                   LIMIT $2
//               ) f
//           ),
//           '[]'::json
//       ) as "features!: Json<Vec<FeatureRow>>"
//       FROM app.collections c
//       WHERE c.slug = $1
//             "#,
//             params.slug,
//             params.limit.map(|l| l as i64)
//         )
//         .fetch_optional(executor)
//         .await
//         .map(|opt| opt.map(|json| json.0))
//     }
// }
