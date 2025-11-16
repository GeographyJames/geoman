use domain::Collection;

use crate::{
    errors::RepositoryError,
    postgres::traits::{SelectAll, SelectOne},
};

impl SelectAll for Collection {
    async fn select_all<'e, E>(executor: E) -> Result<Vec<Self>, RepositoryError>
    where
        E: sqlx::PgExecutor<'e>,
    {
        sqlx::query_as!(
            Collection,
            "SELECT id, title, slug, description FROM app.collections"
        )
        .fetch_all(executor)
        .await
        .map_err(RepositoryError::from)
    }
}

impl SelectOne for Collection {
    type Id<'a> = &'a str;
    async fn select_one<'a, 'e, E>(
        executor: E,
        slug: Self::Id<'a>,
    ) -> Result<Option<Self>, RepositoryError>
    where
        E: sqlx::PgExecutor<'e>,
    {
        sqlx::query_as!(
            Collection,
            "SELECT id, title, slug, description FROM app.collections WHERE slug = $1",
            slug
        )
        .fetch_optional(executor)
        .await
        .map_err(RepositoryError::from)
    }
}
