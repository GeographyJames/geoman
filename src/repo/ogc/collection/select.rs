use crate::repo::{ogc::CollectionRow, traits::SelectAll};

impl SelectAll for CollectionRow {
    async fn select_all<'e, E>(executor: E) -> Result<Vec<Self>, sqlx::Error>
    where
        Self: Sized,
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
