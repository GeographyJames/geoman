use domain::Project;

use crate::postgres::traits::SelectAll;

impl SelectAll for Project {
    async fn select_all<'e, E>(executor: E) -> Result<Vec<Self>, sqlx::Error>
    where
        Self: Sized,
        E: sqlx::PgExecutor<'e>,
    {
        sqlx::query_as!(Project, "SELECT id, name, slug FROM app.projects")
            .fetch_all(executor)
            .await
    }
}
