use domain::{Feature, FeatureId};
use futures::StreamExt;
use serde_json::{Map, Value};
use sqlx::{prelude::FromRow, types::Json};
use std::{
    collections::HashMap,
    sync::{LazyLock, Mutex},
};

use crate::repo::{
    RepositoryError, StreamItem,
    features::{SelectAllParams, SelectOneParams},
    traits::{SelectAllWithParamsStreaming, SelectOneWithParams},
};

const ROWS: &str = r#"gid as id,
               ST_AsGeoJSON(ST_Transform(geom, $1))::json as geom,
               to_jsonb(t) - 'gid' -'geom' as "properties",
               COUNT(*) OVER() as number_matched"#;

static QUERY_CACHE: LazyLock<Mutex<HashMap<(String, String), &'static str>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

#[derive(FromRow)]
struct FeatureRow {
    id: i32,
    geom: Json<geojson::Geometry>,
    properties: Json<Map<String, Value>>,
    number_matched: i64,
}

impl FeatureRow {
    pub fn into_feature(self) -> Feature {
        let Self {
            id,
            geom,
            properties,
            ..
        } = self;
        Feature {
            id,
            geom: geom.0,
            properties: properties.0,
        }
    }
}

impl SelectOneWithParams for Feature {
    type Params<'a> = &'a SelectOneParams<'a>;
    type Id<'a> = FeatureId;

    async fn select_one_with_params<'a, 'e, E>(
        executor: &'e E,
        id: Self::Id<'a>,
        params: Self::Params<'a>,
    ) -> Result<Option<Self>, crate::repo::RepositoryError>
    where
        &'e E: sqlx::PgExecutor<'e>,
    {
        let SelectOneParams { schema, table, crs } = params;
        sqlx::query_as::<_, FeatureRow>(&format!(
            r#"
        SELECT {ROWS}
          FROM "{}"."{}" t
          WHERE gid = $2"#,
            schema,
            table.as_ref()
        ))
        .bind(crs.as_srid())
        .bind(id.0)
        .fetch_optional(executor)
        .await
        .map(|opt| opt.map(|res| res.into_feature()))
        .map_err(RepositoryError::from)
    }
}

impl SelectAllWithParamsStreaming for Feature {
    type Params<'a> = SelectAllParams;

    fn select_all_with_params_streaming<'a>(
        executor: crate::repo::PoolWrapper,
        params: Self::Params<'a>,
    ) -> impl futures::Stream<Item = Result<crate::repo::StreamItem<Self>, RepositoryError>> + use<>
    where
        Self: Sized,
    {
        let SelectAllParams {
            schema,
            table,
            limit,
            bbox,
            bbox_crs,
            crs,
            offset,
        } = params;
        let bbox = bbox.map(|bbox| match bbox {
            ogcapi_types::common::Bbox::Bbox2D(bbox) => bbox,
            ogcapi_types::common::Bbox::Bbox3D(bbox) => [bbox[0], bbox[1], bbox[3], bbox[4]],
        });
        let cache_key = (schema.to_string(), table.as_ref().to_string());

        // ToDo - Here we create the query and cache it so we can use it in the streaming response which is kind of annoying.
        // We also have an inefficiency where we select the geometry and serialise it go Json and then discard it because we
        // don't know in advance which columns we require.

        // Check cache first
        let query: &'static str = {
            let mut cache = QUERY_CACHE.lock().unwrap();

            if let Some(&cached_query) = cache.get(&cache_key) {
                cached_query
            } else {
                // Build and cache the query
                let query = format!(
                    r#"
        SELECT {ROWS}
          FROM "{}"."{}" t
          WHERE ($2::float IS NULL OR (geom && ST_Transform(ST_MakeEnvelope($2, $3, $4, $5, $6), ST_SRID(geom))))
          ORDER BY gid
          LIMIT $7
          OFFSET $8
        "#,
                    schema,
                    table.as_ref()
                );
                // Leak it to make it 'static
                let leaked_query: &'static str = Box::leak(query.into_boxed_str());
                cache.insert(cache_key, leaked_query);
                leaked_query
            }
        };
        sqlx::query_as::<_, FeatureRow>(query)
            .bind(crs.as_srid())
            .bind(bbox.map(|bbox| bbox[0]))
            .bind(bbox.map(|bbox| bbox[1]))
            .bind(bbox.map(|bbox| bbox[2]))
            .bind(bbox.map(|bbox| bbox[3]))
            .bind(bbox_crs.unwrap_or_default().as_srid())
            .bind(limit.map(|l| l as i64))
            .bind(offset.unwrap_or_default() as i32)
            .fetch(executor)
            .map(|res| {
                let row = res?;
                let number_matched = row.number_matched;
                let item: Feature = row.into_feature();
                Ok(StreamItem {
                    item,
                    number_matched,
                })
            })
    }
}
