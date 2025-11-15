use futures::Stream;
use sqlx::PgPool;

use crate::postgres::{
    PoolWrapper,
    traits::{SelectAll, SelectAllWithParamsStreaming, SelectOne},
};
/// Appplication repository
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

    #[tracing::instrument(skip(self, params))]
    pub fn select_all_with_params_streaming<T>(
        &self,
        params: T::Params,
    ) -> impl Stream<Item = Result<T, sqlx::Error>> + use<T>
    where
        T: SelectAllWithParamsStreaming,
    {
        let executor = PoolWrapper(self.db_pool.clone());
        T::select_all_with_params_streaming(executor, params)
    }
}
