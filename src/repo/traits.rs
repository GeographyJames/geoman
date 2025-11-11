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

pub trait SelectOneWithParams {
    type Id<'a>;
    type Params<'a>;

    #[allow(async_fn_in_trait)]
    async fn select_one_with_params<'a, 'e, E>(
        executor: E,
        id: Self::Id<'a>,
        params: Self::Params<'a>,
    ) -> Result<Option<Self>, sqlx::Error>
    where
        Self: Sized,
        E: sqlx::PgExecutor<'e>;
}
