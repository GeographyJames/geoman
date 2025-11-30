use clerk_rs::validators::authorizer::ClerkJwt;
use domain::KeyId;
/// Appplication repository
use futures::Stream;
use sqlx::PgPool;

use crate::repo::{
    PoolWrapper, RepositoryError, StreamItem,
    traits::{
        Insert, SelectAll, SelectAllWithParams, SelectAllWithParamsStreaming, SelectOne,
        SelectOneWithParams,
    },
};

pub struct PostgresRepo {
    pub db_pool: PgPool,
}

impl PostgresRepo {
    pub fn new(db_pool: PgPool) -> Self {
        Self { db_pool }
    }

    #[tracing::instrument(skip(self))]
    pub async fn select_all<T>(&self) -> Result<Vec<T>, RepositoryError>
    where
        T: SelectAll,
    {
        T::select_all(&self.db_pool).await
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn select_one<T, ID>(&self, id: ID) -> Result<Option<T>, RepositoryError>
    where
        T: SelectOne<ID>,
    {
        T::select_one(&self.db_pool, id).await
    }

    #[tracing::instrument(skip(self, params))]
    pub fn select_all_with_params_streaming<'a, T>(
        &self,
        params: T::Params<'a>,
    ) -> impl Stream<Item = Result<StreamItem<T>, RepositoryError>> + use<T>
    where
        T: SelectAllWithParamsStreaming,
    {
        let executor = PoolWrapper(self.db_pool.clone());
        T::select_all_with_params_streaming(executor, params)
    }

    #[tracing::instrument(skip(self, params))]
    pub async fn select_all_with_params<'a, T>(
        &'a self,
        params: T::Params<'a>,
    ) -> Result<(Vec<T>, T::MetaData<'a>), RepositoryError>
    where
        T: SelectAllWithParams,
    {
        T::select_all_with_params(&self.db_pool, params).await
    }

    #[tracing::instrument(skip(self, params, id))]
    pub async fn select_one_with_params<'a, T, ID>(
        &'a self,
        id: ID,
        params: T::Params<'a>,
    ) -> Result<Option<T>, RepositoryError>
    where
        T: SelectOneWithParams<ID>,
    {
        T::select_one_with_params(&self.db_pool, id, params).await
    }

    #[tracing::instrument(skip(self, item))]
    pub async fn insert<T>(&self, item: &T) -> Result<T::Id, RepositoryError>
    where
        T: Insert,
    {
        item.insert(&self.db_pool).await
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn revoke_api_key(&self, id: KeyId, user: &ClerkJwt) -> Result<(), RepositoryError> {
        sqlx::query_scalar!(
            "
        UPDATE app.api_keys
        SET revoked = NOW()
        WHERE id = $1
        AND user_id = (SELECT id FROM app.users WHERE clerk_id = $2) RETURNING id",
            id.0,
            user.sub
        )
        .fetch_one(&self.db_pool)
        .await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn renew_api_key(&self, id: KeyId, user: &ClerkJwt) -> Result<(), RepositoryError> {
        sqlx::query_scalar!(
            "
        UPDATE app.api_keys
        SET expiry = (NOW() + INTERVAL '6 months')
        WHERE id = $1 AND user_id = (
             SELECT id FROM app.users WHERE clerk_id = $2
             )
        RETURNING id",
            id.0,
            user.sub
        )
        .fetch_one(&self.db_pool)
        .await?;
        Ok(())
    }
}
