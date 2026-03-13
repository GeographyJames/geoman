use crate::repo::{RepositoryError, traits::Insert};
use qgis::project::QgisProject;

impl Insert for QgisProject {
    type Id = String;

    async fn insert<'a, A>(&self, executor: A) -> Result<Self::Id, RepositoryError>
    where
        Self: Sized,
        A: sqlx::Acquire<'a, Database = sqlx::Postgres>,
    {
        let mut tx = executor.begin().await?;

        sqlx::query!(
            "DELETE FROM public.qgis_projects WHERE figure_id = $1 AND low_res = true",
            self.figure_id
        )
        .execute(&mut *tx)
        .await?;

        sqlx::query!(
            "INSERT INTO public.qgis_projects (name, metadata, content, figure_id, low_res) VALUES ($1, $2, $3, $4, $5) ON CONFLICT (name) DO NOTHING",
            self.name,
            serde_json::to_value(&self.metadata).map_err(|e| RepositoryError::UnexpectedError(
                anyhow::anyhow!("failed to serialize qgis project metadata: {}", e).into()
            ))?,
            self.content,
            self.figure_id,
            self.low_res
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(self.name.clone())
    }
}
