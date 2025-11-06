use crate::repo::{
    ogc::CollectionRow,
    traits::{SelectAll, SelectBySlug},
};

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

impl SelectBySlug for CollectionRow {
    async fn select_by_slug<'e, E>(executor: E, slug: &str) -> Result<Option<Self>, sqlx::Error>
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
