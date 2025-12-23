use domain::{ProjectId, UserId, enums::Visibility, project::ProjectInputDto};
use sqlx::{Acquire, Postgres};

use crate::repo::traits::Insert;

impl Insert for (&ProjectInputDto, UserId) {
    type Id = ProjectId;

    async fn insert<'a, A>(&self, conn: A) -> Result<Self::Id, crate::repo::RepositoryError>
    where
        A: Acquire<'a, Database = Postgres>,
    {
        let mut tx = conn.begin().await?;
        let (dto, id) = self;
        let project_id = sqlx::query_scalar!(
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
            VALUES( $1, $2, $3, $4, $5, $5, $5, $6, (SELECT team_id from app.users WHERE id =$5 ))

            RETURNING id AS "id: ProjectId"
            "#,
            dto.name.as_ref(),
            &dto.visibility as &Visibility,
            dto.country_code.alpha2(),
            dto.crs_srid,
            id.0,
            dto.slug.as_ref()
        )
        .fetch_one(&mut *tx)
        .await?;

        if let Some(techs) = &dto.technologies {
            let tech_ids: Vec<i32> = techs.iter().map(|t| t.0).collect();
            // Insert technologies
            sqlx::query!(
                r#"
      INSERT INTO app.project_technologies (project_id, technology_id)
      SELECT $1, UNNEST($2::integer[])
      "#,
                project_id.0,
                &tech_ids as &[i32]
            )
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;

        Ok(project_id)
    }
}
