pub trait SelectAll {
    #[allow(async_fn_in_trait)]
    async fn select_all<'a, E>(executor: E) -> Result<Vec<Self>, anyhow::Error>
    where
        Self: Sized,
        E: sqlx::PgExecutor<'a>;
}

pub trait Migrate {
    #[allow(async_fn_in_trait)]
    async fn migrate<'a, E>(self, executor: E) -> Result<(), anyhow::Error>
    where
        Self: Sized,
        E: sqlx::PgExecutor<'a>;
}
