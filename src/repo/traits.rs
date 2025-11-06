pub trait SelectAll {
    #[allow(async_fn_in_trait)]
    async fn select_all<'e, E>(executor: E) -> Result<Vec<Self>, sqlx::Error>
    where
        Self: Sized,
        E: sqlx::PgExecutor<'e>;
}

pub trait SelectBySlug {
    #[allow(async_fn_in_trait)]
    async fn select_by_slug<'e, E>(executor: E, slug: &str) -> Result<Option<Self>, sqlx::Error>
    where
        Self: Sized,
        E: sqlx::PgExecutor<'e>;
}
