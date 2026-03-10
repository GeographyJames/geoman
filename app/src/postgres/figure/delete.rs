use domain::{FigureId, enums::Status};

use crate::{postgres::PostgresRepo, repo::RepositoryError};

impl PostgresRepo {
    pub async fn delete_figure(&self, id: FigureId) -> Result<(), RepositoryError> {
        sqlx::query!(
            r#"UPDATE app.figures SET status = $1 WHERE id = $2"#,
            Status::Deleted as Status,
            id.0
        )
        .execute(&self.db_pool)
        .await?;
        Ok(())
    }
}
