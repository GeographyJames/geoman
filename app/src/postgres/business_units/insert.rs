use domain::{BusinessUnitId, UserId};

use crate::{handlers::api::business_units::BusinessUnitInputPayload, repo::traits::Insert};

impl Insert for (BusinessUnitInputPayload, UserId) {
    type Id = BusinessUnitId;

    async fn insert<'a, A>(&self, executor: A) -> Result<Self::Id, crate::repo::RepositoryError>
    where
        Self: Sized,
        A: sqlx::Acquire<'a, Database = sqlx::Postgres>,
    {
        let mut conn = executor.acquire().await?;
        let (dto, user) = self;
        let res = sqlx::query!(
            "
        INSERT INTO app.business_units(name, added_by, last_updated_by) VALUES ($1, $2, $3) RETURNING id
        ",
            dto.name,
            user.0,
            user.0
        )
        .fetch_one(&mut *conn)
        .await?;
        Ok(BusinessUnitId(res.id))
    }
}
