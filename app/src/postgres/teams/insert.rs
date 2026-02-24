use domain::{TeamId, UserId};

use crate::{handlers::api::teams::TeamInputPayload, repo::traits::Insert};

impl Insert for (TeamInputPayload, UserId) {
    type Id = TeamId;

    async fn insert<'a, A>(&self, executor: A) -> Result<Self::Id, crate::repo::RepositoryError>
    where
        Self: Sized,
        A: sqlx::Acquire<'a, Database = sqlx::Postgres>,
    {
        let mut conn = executor.acquire().await?;
        let (dto, user) = self;
        let res = sqlx::query!(
            "
        INSERT INTO app.teams(name, business_unit_id, added_by, last_updated_by) VALUES ($1, $2, $3, $4) RETURNING id
        ",
            dto.name,
            dto.business_unit.0,
            user.0,
            user.0
        )
        .fetch_one(&mut *conn)
        .await?;
        Ok(TeamId(res.id))
    }
}
