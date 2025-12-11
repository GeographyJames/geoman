#![allow(async_fn_in_trait)]
use futures::Stream;

use crate::repo::{PoolWrapper, RepositoryError, StreamItem};

pub trait SelectAll {
    async fn select_all<'a, E>(executor: &'a E) -> Result<Vec<Self>, RepositoryError>
    where
        Self: Sized,
        &'a E: sqlx::PgExecutor<'a>;
}

pub trait SelectOne<ID> {
    async fn select_one<'a, E>(executor: &'a E, id: ID) -> Result<Option<Self>, RepositoryError>
    where
        Self: Sized,
        &'a E: sqlx::PgExecutor<'a>;
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

    async fn select_all_with_params<'a, E>(
        executor: &'a E,
        params: Self::Params<'a>,
    ) -> Result<(Vec<Self>, Self::MetaData<'a>), RepositoryError>
    where
        Self: Sized,
        &'a E: sqlx::PgExecutor<'a>;
}

pub trait SelectOneWithParams<ID> {
    type Params<'a>;
    async fn select_one_with_params<'a, E>(
        executor: &'a E,
        id: ID,
        params: Self::Params<'a>,
    ) -> Result<Option<Self>, RepositoryError>
    where
        Self: Sized,
        &'a E: sqlx::PgExecutor<'a>;
}

pub trait Insert {
    type Id;
    async fn insert<'a, E>(&self, executor: &'a E) -> Result<Self::Id, RepositoryError>
    where
        Self: Sized,
        &'a E: sqlx::PgExecutor<'a>;
}

pub trait Update {
    type Id;
    async fn update<'a, E>(&self, executor: &'a E) -> Result<Self::Id, RepositoryError>
    where
        Self: Sized,
        &'a E: sqlx::PgExecutor<'a>;
}
