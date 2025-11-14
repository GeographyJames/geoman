use ogc::types::common::CollectionRow;

use crate::traits::{SelectAll, SelectOne};

impl SelectAll for CollectionRow {
    async fn select_all<'e, E>(executor: E) -> Result<Vec<Self>, sqlx::Error>
    where
        E: sqlx::PgExecutor<'e>,
    {
        sqlx::query_as!(
            CollectionRow,
            "SELECT id, title, slug, description FROM app.collections"
        )
        .fetch_all(executor)
        .await
    }
}

impl SelectOne for CollectionRow {
    type Id<'a> = &'a str;
    async fn select_one<'a, 'e, E>(
        executor: E,
        slug: Self::Id<'a>,
    ) -> Result<Option<Self>, sqlx::Error>
    where
        E: sqlx::PgExecutor<'e>,
    {
        sqlx::query_as!(
            CollectionRow,
            "SELECT id, title, slug, description FROM app.collections WHERE slug = $1",
            slug
        )
        .fetch_optional(executor)
        .await
    }
}
