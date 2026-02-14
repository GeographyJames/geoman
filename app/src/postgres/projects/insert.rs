use domain::{ProjectId, UserId, enums::Visibility, project::ProjectInputDto};
use sqlx::{Acquire, Postgres};

use crate::repo::traits::Insert;

impl Insert for (&ProjectInputDto, UserId) {
    type Id = ProjectId;

    async fn insert<'a, A>(&self, conn: A) -> Result<Self::Id, crate::repo::RepositoryError>
    where
        A: Acquire<'a, Database = Postgres>,
    {
        let mut conn = conn.acquire().await?;
        let (dto, id) = self;
        let project_id = sqlx::query_scalar!(
            r#"
            INSERT INTO app.projects (
                name,
                visibility,
                crs_srid,
                owner,
                added_by,
                last_updated_by,
                slug,
                team_id
            )
            VALUES( $1, $2, $3, $4, $4, $4, $5, (SELECT team_id from app.users WHERE id =$4 ))

            RETURNING id AS "id: ProjectId"
            "#,
            dto.name.as_ref(),
            &dto.visibility as &Visibility,
            dto.crs_srid,
            id.0,
            dto.slug.as_ref()
        )
        .fetch_one(&mut *conn)
        .await?;

        Ok(project_id)
    }
}
