use domain::{TeamId, UserId};
use sqlx::{Acquire, Postgres};

use crate::{handlers::api::teams::TeamUpdatePayload, repo::traits::Update};

impl Update for (TeamUpdatePayload, TeamId, UserId) {
    type Id = TeamId;

    async fn update<'a, A>(&self, conn: A) -> Result<Self::Id, crate::repo::RepositoryError>
    where
        Self: Sized,
        A: Acquire<'a, Database = Postgres>,
    {
        let mut executor = conn.acquire().await?;
        let (payload, team_id, user_id) = self;
        let res = sqlx::query!(
            r#"
            UPDATE app.teams
            SET name = COALESCE($1, name),
                business_unit_id = CASE WHEN $2 THEN $3 ELSE business_unit_id END,
                last_updated = NOW(),
                last_updated_by = $5
            WHERE id = $4
            RETURNING id
            "#,
            payload.name,
            payload.business_unit.is_some(),
            payload.business_unit.clone().flatten().map(|id| id.0),
            team_id.0,
            user_id.0
        )
        .fetch_one(&mut *executor)
        .await?;
        Ok(TeamId(res.id))
    }
}
