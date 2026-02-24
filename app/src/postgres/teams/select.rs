use domain::{BusinessUnitId, Team, TeamId};

use crate::repo::traits::SelectAll;

impl SelectAll for Team {
    async fn select_all<'a, E>(executor: &'a E) -> Result<Vec<Self>, crate::repo::RepositoryError>
    where
        Self: Sized,
        &'a E: sqlx::PgExecutor<'a>,
    {
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
        .fetch_all(executor)
        .await?;
        Ok(res)
    }
}
