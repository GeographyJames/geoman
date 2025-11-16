use futures::Stream;

use crate::{errors::RepositoryError, postgres::pool_wrapper::PoolWrapper};

pub trait SelectAll {
    #[allow(async_fn_in_trait)]
    async fn select_all<'e, E>(executor: E) -> Result<Vec<Self>, RepositoryError>
    where
        Self: Sized,
        E: sqlx::PgExecutor<'e>;
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
        E: sqlx::PgExecutor<'e>;
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

pub trait SelectAllStreaiming {
    fn select_all_streaming(
        executor: PoolWrapper,
    ) -> impl Stream<Item = Result<Self, RepositoryError>> + use<Self>
    where
        Self: Sized;
}
