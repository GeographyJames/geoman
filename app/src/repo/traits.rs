use futures::Stream;

use crate::repo::{PoolWrapper, RepositoryError, StreamItem};

pub trait SelectAll {
    #[allow(async_fn_in_trait)]
    async fn select_all<'e, E>(executor: &'e E) -> Result<Vec<Self>, RepositoryError>
    where
        Self: Sized,
        &'e E: sqlx::PgExecutor<'e>;
}

pub trait SelectOne {
    type Id<'a>;

    #[allow(async_fn_in_trait)]
    async fn select_one<'a, 'e, E>(
        executor: &'e E,
        id: Self::Id<'a>,
    ) -> Result<Option<Self>, RepositoryError>
    where
        Self: Sized,
        &'e E: sqlx::PgExecutor<'e>;
}

pub trait SelectAllWithParamsStreaming {
    type Params<'a>;

    fn select_all_with_params_streaming<'a>(
        executor: PoolWrapper,
        params: Self::Params<'a>,
    ) -> impl Stream<Item = Result<StreamItem<Self>, RepositoryError>> + use<Self>
    where
        Self: Sized;
}

pub trait SelectAllWithParams {
    type Params<'a>;
    type MetaData<'a>;

    async fn select_all_with_params<'a, 'e, E>(
        executor: &'e E,
        params: Self::Params<'a>,
    ) -> Result<(Vec<Self>, Self::MetaData<'a>), RepositoryError>
    where
        Self: Sized,
        &'e E: sqlx::PgExecutor<'e>;
}

pub trait SelectOneWithParams {
    type Params<'a>;
    type Id<'a>;
    async fn select_one_with_params<'a, 'e, E>(
        executor: &'e E,
        id: Self::Id<'a>,
        params: Self::Params<'a>,
    ) -> Result<Option<Self>, RepositoryError>
    where
        Self: Sized,
        &'e E: sqlx::PgExecutor<'e>;
}
