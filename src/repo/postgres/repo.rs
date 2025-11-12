use crate::repo::models::ogc::FeatureRow;
use crate::repo::traits::{SelectAll, SelectAllWithParams, SelectOne, SelectOneWithParams};
use futures::{Stream, StreamExt};
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

    #[tracing::instrument(skip(self, params))]
    pub async fn select_all_with_params<'a, T>(
        &self,
        params: T::Params<'a>,
    ) -> Result<Vec<T>, sqlx::Error>
    where
        T: SelectAllWithParams,
    {
        T::select_all_with_params(&self.db_pool, params).await
    }

    #[tracing::instrument(skip(self, params))]
    pub fn select_all_with_params_streaming(
        &self,
        params: DbQueryParams,
    ) -> impl Stream<Item = Result<FeatureRow, sqlx::Error>> + '_ {
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
            params.limit
        )
        .fetch(&self.db_pool)
        .map(|res| res.map(|json| json.0))
    }
}

pub struct DbQueryParams {
    slug: String,
    limit: Option<i64>,
}
