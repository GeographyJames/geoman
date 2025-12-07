use domain::{ProjectId, enums::Visibility, project::ProjectInputDto};

use crate::repo::traits::Insert;

impl Insert for (&ProjectInputDto, &str) {
    type Id = ProjectId;

    async fn insert<'a, E>(&self, executor: &'a E) -> Result<Self::Id, crate::repo::RepositoryError>
    where
        &'a E: sqlx::PgExecutor<'a>,
    {
        let (dto, clerk_id) = self;
        sqlx::query_scalar!(
            r#"
            WITH user_data AS (
                INSERT INTO app.users (clerk_id, first_name, last_name, team_id)
                VALUES ($1, 'New', 'User', -1)
                ON CONFLICT (clerk_id) DO UPDATE
                SET clerk_id = EXCLUDED.clerk_id
                RETURNING id, team_id
            )
            INSERT INTO app.projects (
                name,
                visibility,
                country_code,
                crs_srid,
                owner,
                added_by,
                last_updated_by,
                slug,
                team_id
            )
            SELECT $2, $3, $4, $5, user_data.id, user_data.id, user_data.id, $6, user_data.team_id
            FROM user_data
            RETURNING id AS "id: ProjectId"
            "#,
            clerk_id,
            dto.name.as_ref(),
            &dto.visibility as &Visibility,
            dto.country_code.alpha2(),
            dto.crs_srid,
            dto.slug.as_ref()
        )
        .fetch_one(executor)
        .await
        .map_err(Into::into)
    }
}
