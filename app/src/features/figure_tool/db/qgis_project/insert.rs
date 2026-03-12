use sqlx::PgPool;

use crate::{
    qgis::project::QgisProject,
    repo::{Insert, RepositoryError},
};

impl<'a> Insert<&'a PgPool, String> for QgisProject {
    async fn insert(&self, executor: &'a PgPool) -> Result<String, crate::repo::RepositoryError> {
        let mut tx = executor.begin().await?;

        sqlx::query!(
            r#"
        DELETE FROM qgis.qgis_projects WHERE figure_id = $1 AND low_res = true"#,
            self.figure_id
        )
        .execute(&mut *tx)
        .await?;

        sqlx::query!(
            r#"INSERT INTO qgis.qgis_projects (name, metadata, content, figure_id, low_res) VALUES ($1, $2, $3, $4, $5)"#,
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
