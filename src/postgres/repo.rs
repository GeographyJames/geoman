use crate::postgres::PoolWrapper;
use crate::postgres::traits::SelectAllWithParamsStreaming;
use crate::postgres::traits::{SelectAll, SelectOne};
use futures::Stream;
use sqlx::PgPool;

pub struct PostgresRepo {
    pub db_pool: PgPool,
}

impl PostgresRepo {
    pub fn new(db_pool: PgPool) -> Self {
        Self { db_pool }
    }

    pub fn pool_wrapper(&self) -> PoolWrapper {
        PoolWrapper(self.db_pool.clone())
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

    #[tracing::instrument(skip(self, params))]
    pub fn select_all_with_params_streaming<T>(
        &self,
        params: T::Params,
    ) -> impl Stream<Item = Result<T, sqlx::Error>> + use<T>
    where
        T: SelectAllWithParamsStreaming,
    {
        let executor = self.pool_wrapper();
        T::select_all_with_params_streaming(executor, params)
    }

    // #[tracing::instrument(skip(self, id, params))]
    // pub async fn select_one_with_params<'a, T>(
    //     &self,
    //     id: T::Id<'a>,
    //     params: T::Params<'a>,
    // ) -> Result<Option<T>, sqlx::Error>
    // where
    //     T: SelectOneWithParams,
    // {
    //     T::select_one_with_params(&self.db_pool, id, params).await
    // }

    // #[tracing::instrument(skip(self, params))]
    // pub async fn select_all_with_params<'a, T>(
    //     &self,
    //     params: T::Params<'a>,
    // ) -> Result<Vec<T>, sqlx::Error>
    // where
    //     T: SelectAllWithParams,
    // {
    //     T::select_all_with_params(&self.db_pool, params).await
    // }

    // #[tracing::instrument(skip(self, params))]
    // pub async fn select_all_features_by_collection<'a>(
    //     &self,
    //     params: &SelectAllParams<'a>,
    // ) -> Result<Option<Vec<FeatureRow>>, sqlx::Error> {
    //     FeatureRow::select_all_features_by_collection(&self.db_pool, params).await
    // }
}
