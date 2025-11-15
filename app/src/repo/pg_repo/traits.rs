use futures::Stream;

use crate::repo::pg_repo::PoolWrapper;

pub trait SelectAll {
    #[allow(async_fn_in_trait)]
    async fn select_all<'e, E>(executor: E) -> Result<Vec<Self>, sqlx::Error>
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
    ) -> Result<Option<Self>, sqlx::Error>
    where
        Self: Sized,
        E: sqlx::PgExecutor<'e>;
}

pub trait SelectAllWithParamsStreaming {
    type Params;

    fn select_all_with_params_streaming(
        executor: PoolWrapper,
        params: Self::Params,
    ) -> impl Stream<Item = Result<Self, sqlx::Error>> + use<Self>
    where
        Self: Sized;
}
