use domain::{BusinessUnitId, UserId};
use sqlx::{Acquire, Postgres};

use crate::{handlers::api::business_units::BusinessUnitUpdatePayload, repo::traits::Update};

impl Update for (BusinessUnitUpdatePayload, BusinessUnitId, UserId) {
    type Id = BusinessUnitId;

    async fn update<'a, A>(&self, conn: A) -> Result<Self::Id, crate::repo::RepositoryError>
    where
        Self: Sized,
        A: Acquire<'a, Database = Postgres>,
    {
        let mut executor = conn.acquire().await?;
        let (payload, bu_id, user_id) = self;
        let res = sqlx::query!(
            r#"
            UPDATE app.business_units
            SET name = COALESCE($1, name),
                last_updated = NOW(),
                last_updated_by = $3
            WHERE id = $2
            RETURNING id
            "#,
            payload.name,
            bu_id.0,
            user_id.0
        )
        .fetch_one(&mut *executor)
        .await?;
        Ok(BusinessUnitId(res.id))
    }
}
