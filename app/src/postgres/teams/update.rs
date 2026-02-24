use domain::TeamId;
use sqlx::{Acquire, Postgres};

use crate::{handlers::api::teams::TeamUpdatePayload, repo::traits::Update};

impl Update for (TeamUpdatePayload, TeamId) {
    type Id = TeamId;

    async fn update<'a, A>(&self, conn: A) -> Result<Self::Id, crate::repo::RepositoryError>
    where
        Self: Sized,
        A: Acquire<'a, Database = Postgres>,
    {
        let mut executor = conn.acquire().await?;
        let (payload, team_id) = self;
        let res = sqlx::query!(
            r#"
            UPDATE app.teams
            SET name = COALESCE($1, name),
                business_unit_id = COALESCE($2, business_unit_id)
            WHERE id = $3
            RETURNING id
            "#,
            payload.name,
            payload.business_unit.map(|id| id.0),
            team_id.0
        )
        .fetch_one(&mut *executor)
        .await?;
        Ok(TeamId(res.id))
    }
}
