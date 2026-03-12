use domain::{BusinessUnit, BusinessUnitId};

use crate::repo::traits::SelectAll;

impl SelectAll for BusinessUnit {
    async fn select_all<'a, A>(executor: A) -> Result<Vec<Self>, crate::repo::RepositoryError>
    where
        Self: Sized,
        A: sqlx::Acquire<'a, Database = sqlx::Postgres>,
    {
        let mut conn = executor.acquire().await?;
        let res = sqlx::query_as!(
            BusinessUnit,
            r#"SELECT id as "id: BusinessUnitId", name FROM app.business_units ORDER BY name ASC"#
        )
        .fetch_all(&mut *conn)
        .await?;
        Ok(res)
    }
}
