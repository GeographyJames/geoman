use sqlx::PgExecutor;

use crate::{domain::entities::Figure, repo::Delete};

impl<REPO, ID> Delete<REPO, ID> for Figure
where
    for<'a> REPO: PgExecutor<'a>,
    ID: AsRef<i32>,
{
    async fn delete(executor: REPO, id: &ID) -> Result<(), crate::repo::RepositoryError> {
        sqlx::query!(
            "UPDATE app.figures SET status='deleted' WHERE id = $1",
            id.as_ref()
        )
        .execute(executor)
        .await?;
        Ok(())
    }
}
