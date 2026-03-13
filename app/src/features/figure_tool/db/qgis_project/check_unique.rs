use sqlx::PgExecutor;

use crate::{features::figure_tool::dtos::QgisProjectName, repo::RepositoryError};

impl QgisProjectName {
    pub async fn check_unique<'a, E: PgExecutor<'a>>(
        &self,
        executor: E,
    ) -> Result<Option<QgisProjectName>, RepositoryError> {
        let res = sqlx::query!(
            "SELECT name FROM public.qgis_projects WHERE name = $1",
            self.0
        )
        .fetch_optional(executor)
        .await?
        .map(|record| record.name);
        Ok(res.map(QgisProjectName))
    }
}
