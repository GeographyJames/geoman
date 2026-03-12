use domain::{BusinessUnitId, Team, TeamId};

use crate::repo::traits::SelectAll;

impl SelectAll for Team {
    async fn select_all<'a, A>(executor: A) -> Result<Vec<Self>, crate::repo::RepositoryError>
    where
        Self: Sized,
        A: sqlx::Acquire<'a, Database = sqlx::Postgres>,
    {
        let mut conn = executor.acquire().await?;
        let res = sqlx::query_as!(
            Team,
            r#"
        SELECT t.id as "id: TeamId",
               t.name,
               t.business_unit_id as "business_unit_id: BusinessUnitId"


        FROM app.teams t
        WHERE t.id > 0
        ORDER BY t.name ASC
        "#
        )
        .fetch_all(&mut *conn)
        .await?;
        Ok(res)
    }
}
