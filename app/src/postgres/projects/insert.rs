use clerk_rs::validators::authorizer::ClerkJwt;
use domain::{ProjectId, UserId, enums::Visibility, project::ProjectInputDto};

use crate::repo::traits::Insert;

impl Insert for (ProjectInputDto, ClerkJwt) {
    type Id = ProjectId;

    async fn insert<'a, E>(&self, executor: &'a E) -> Result<Self::Id, crate::repo::RepositoryError>
    where
        Self: Sized,
        &'a E: sqlx::PgExecutor<'a>,
    {
        let (dto, jwt) = self;
        sqlx::query_scalar!(
            r#"
            WITH user_cte AS (SELECT u.id AS uid FROM app.users u WHERE u.clerk_id = $1)
            INSERT INTO app.projects (
                            name,
                            visibility,
                            country_code,
                            crs_srid,
                            owner,
                            added_by,
                            last_updated_by
                            )   
                 SELECT $2, $3, $4, $5, uid, uid, uid
                 FROM user_cte
                RETURNING id AS "id: ProjectId""#,
            jwt.sub,
            dto.name,
            &dto.visibility as &Visibility,
            dto.country_code.alpha2(),
            dto.crs_srid,
        )
        .fetch_one(executor)
        .await
        .map_err(Into::into)
    }
}
