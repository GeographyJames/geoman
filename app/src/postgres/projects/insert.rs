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
            SELECT $2, $3, $4, $5, user_id, user_id, user_id, $6, user_team_id
            FROM app.upsert_user_and_get_context($1)
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
