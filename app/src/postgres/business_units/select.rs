use domain::{BusinessUnit, BusinessUnitId};

use crate::repo::traits::SelectAll;

impl SelectAll for BusinessUnit {
    async fn select_all<'a, E>(executor: &'a E) -> Result<Vec<Self>, crate::repo::RepositoryError>
    where
        Self: Sized,
        &'a E: sqlx::PgExecutor<'a>,
    {
        let res = sqlx::query_as!(
            BusinessUnit,
            r#"SELECT id as "id: BusinessUnitId", name FROM app.business_units"#
        )
        .fetch_all(executor)
        .await?;
        Ok(res)
    }
}
