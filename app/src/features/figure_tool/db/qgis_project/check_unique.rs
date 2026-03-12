use sqlx::PgExecutor;

use crate::{
    app::features::figure_tool::dtos::figure::QgisProjectName,
    repo::CheckUnique,
};

impl<REPO> CheckUnique<REPO, QgisProjectName> for QgisProjectName
where
    for<'a> REPO: PgExecutor<'a>,
{
    async fn check_unique(
        &self,
        executor: REPO,
    ) -> Result<Option<QgisProjectName>, crate::repo::RepositoryError> {
        let res = sqlx::query!(
            "SELECT name FROM qgis.qgis_projects WHERE name = $1",
            self.0
        )
        .fetch_optional(executor)
        .await?
        .map(|record| record.name);
        Ok(res.map(QgisProjectName))
    }
}
