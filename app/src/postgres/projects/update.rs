use domain::{
    ProjectId, UserId,
    enums::{Status, Visibility},
    project::ProjectUpdateDto,
};
use sqlx::{Acquire, Postgres};

use crate::repo::traits::Update;

impl Update for (&ProjectUpdateDto, UserId) {
    type Id = ProjectId;

    async fn update<'a, E>(&self, conn: E) -> Result<Self::Id, crate::repo::RepositoryError>
    where
        Self: Sized,
        E: Acquire<'a, Database = Postgres>,
    {
        let (dto, user_id) = self;
        let mut tx = conn.begin().await?;

        let name = dto.name.as_ref().map(|n| n.as_ref());
        let slug = dto.slug.as_ref().map(|s| s.as_ref());
        let country_code = dto.country_code.as_ref().map(|c| c.alpha2());

        let crs_srid_provided = dto.crs_srid.is_some();
        let crs_srid_value = dto.crs_srid.flatten();

        let result = sqlx::query!(
            r#"
            UPDATE app.projects
            SET
                status = COALESCE($1, status),
                name = COALESCE($2, name),
                visibility = COALESCE($3, visibility),
                country_code = COALESCE($4, country_code),
                crs_srid = CASE WHEN $5 THEN $6 ELSE crs_srid END,
                slug = COALESCE($7, slug),
                last_updated = NOW(),
                last_updated_by = $8
            WHERE id = $9
            RETURNING id
            "#,
            &dto.status as &Option<Status>,
            name,
            &dto.visibility as &Option<Visibility>,
            country_code,
            crs_srid_provided,
            crs_srid_value,
            slug,
            user_id.0,
            dto.id.0
        )
        .fetch_one(&mut *tx)
        .await?;

        if let Some(techs) = &dto.technologies {
            sqlx::query!(
                r#"
                DELETE FROM app.project_technologies
                WHERE project_id = $1
                "#,
                dto.id.0
            )
            .execute(&mut *tx)
            .await?;

            if !techs.is_empty() {
                let tech_ids: Vec<i32> = techs.iter().map(|t| t.0).collect();
                sqlx::query!(
                    r#"
                    INSERT INTO app.project_technologies (project_id, technology_id)
                    SELECT $1, UNNEST($2::integer[])
                    "#,
                    dto.id.0,
                    &tech_ids as &[i32]
                )
                .execute(&mut *tx)
                .await?;
            }
        }

        tx.commit().await?;

        Ok(ProjectId(result.id))
    }
}
