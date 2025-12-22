use domain::{Technology, TechnologyId};

use crate::repo::traits::SelectAll;

impl SelectAll for Technology {
    async fn select_all<'a, E>(executor: &'a E) -> Result<Vec<Self>, crate::repo::RepositoryError>
    where
        Self: Sized,
        &'a E: sqlx::PgExecutor<'a>,
    {
        sqlx::query_as!(
            Technology,
            r#"SELECT id AS "id: TechnologyId", name, logo_svg FROM app.technologies"#
        )
        .fetch_all(executor)
        .await
        .map_err(Into::into)
    }
}
