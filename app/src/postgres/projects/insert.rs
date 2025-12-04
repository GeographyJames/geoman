use domain::{ProjectId, UserId, enums::Visibility, project::ProjectInputDto};

use crate::repo::traits::Insert;

impl Insert for (ProjectInputDto, UserId) {
    type Id = ProjectId;

    async fn insert<'a, E>(&self, executor: &'a E) -> Result<Self::Id, crate::repo::RepositoryError>
    where
        &'a E: sqlx::PgExecutor<'a>,
    {
        let (dto, user_id) = self;
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
                    VALUES ($1, $2, $3, $4, $5, $5, $5, $6,
                            (SELECT team_id FROM app.users WHERE id = $5)
                )
                RETURNING id AS "id: ProjectId""#,
            dto.name.as_ref(),
            &dto.visibility as &Visibility,
            dto.country_code.alpha2(),
            dto.crs_srid,
            user_id.0,
            dto.slug.as_ref()
        )
        .fetch_one(executor)
        .await
        .map_err(Into::into)
    }
}
