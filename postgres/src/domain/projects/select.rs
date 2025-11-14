use domain::ProjectRow;

use crate::traits::SelectAll;

impl SelectAll for ProjectRow {
    async fn select_all<'e, E>(executor: E) -> Result<Vec<Self>, sqlx::Error>
    where
        Self: Sized,
        E: sqlx::PgExecutor<'e>,
    {
        sqlx::query_as!(ProjectRow, "SELECT id, name, slug FROM app.projects")
            .fetch_all(executor)
            .await
    }
}
