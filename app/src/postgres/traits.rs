use futures::Stream;
use sqlx::PgExecutor;

use crate::{errors::RepositoryError, postgres::pool_wrapper::PoolWrapper};

pub trait SelectAll {
    #[allow(async_fn_in_trait)]
    async fn select_all<'e, E>(executor: E) -> Result<Vec<Self>, RepositoryError>
    where
        Self: Sized,
        E: PgExecutor<'e>;
}

pub trait SelectOne {
    type Id<'a>;

    #[allow(async_fn_in_trait)]
    async fn select_one<'a, 'e, E>(
        executor: E,
        id: Self::Id<'a>,
    ) -> Result<Option<Self>, RepositoryError>
    where
        Self: Sized,
        E: PgExecutor<'e>;
}

pub trait SelectAllWithParamsStreaming {
    type Params;

    fn select_all_with_params_streaming(
        executor: PoolWrapper,
        params: Self::Params,
    ) -> impl Stream<Item = Result<Self, RepositoryError>> + use<Self>
    where
        Self: Sized;
}

pub trait SelectAllWithParams {
    type Params;
    async fn select_all_with_params<'e, E>(
        executor: E,
        params: Self::Params,
    ) -> Result<Vec<Self>, RepositoryError>
    where
        Self: Sized,
        E: PgExecutor<'e>;
}
