use crate::repo::traits::{SelectAll, SelectOne, SelectOneWithParams};
use futures::Stream;
use sqlx::PgPool;
use sqlx::types::Json;

pub struct PostgresRepo {
    pub db_pool: PgPool,
}

impl PostgresRepo {
    pub fn new(db_pool: PgPool) -> Self {
        Self { db_pool }
    }

    #[tracing::instrument(skip(self))]
    pub async fn select_all<T>(&self) -> Result<Vec<T>, sqlx::Error>
    where
        T: SelectAll,
    {
        T::select_all(&self.db_pool).await
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn select_one<'a, T>(&self, id: T::Id<'a>) -> Result<Option<T>, sqlx::Error>
    where
        T: SelectOne,
    {
        T::select_one(&self.db_pool, id).await
    }

    #[tracing::instrument(skip(self, id, params))]
    pub async fn select_one_with_params<'a, T>(
        &self,
        id: T::Id<'a>,
        params: T::Params<'a>,
    ) -> Result<Option<T>, sqlx::Error>
    where
        T: SelectOneWithParams,
    {
        T::select_one_with_params(&self.db_pool, id, params).await
    }

    #[tracing::instrument(skip(self, collection_id, limit))]
    pub fn select_features_streaming(
        &self,
        collection_id: i32,
        limit: Option<usize>,
    ) -> impl Stream<Item = Result<Json<geojson::Feature>, sqlx::Error>> + '_ {
        sqlx::query_scalar!(
            r#"
            SELECT ST_AsGeoJSON(t.*, id_column => 'id')::jsonb as "f!: Json<geojson::Feature>"
            FROM (
                SELECT id, name, ST_Transform(geom, 4326) as geom
                FROM app.features
                WHERE collection_id = $1 AND status = 'ACTIVE'
                ORDER BY id
                LIMIT $2 
                ) 
            as t(id, name, geom)
            "#,
            collection_id,
            limit.map(|l| l as i64)
        )
        .fetch(&self.db_pool)
    }
}
