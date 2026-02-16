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
        let mut conn = conn.acquire().await?;

        let mut name = dto.name.as_ref().map(|n| n.as_ref().to_string());
        let slug = dto.slug.as_ref().map(|s| s.as_ref());

        let crs_srid_provided = dto.crs_srid.is_some();
        let crs_srid_value = dto.crs_srid.flatten();
        if let Some(ref status) = dto.status
            && status == &Status::Deleted
        {
            name = Some(uuid::Uuid::new_v4().to_string())
        }

        let result = sqlx::query!(
            r#"
            UPDATE app.projects
            SET
                status = COALESCE($1, status),
                name = COALESCE($2, name),
                visibility = COALESCE($3, visibility),
                crs_srid = CASE WHEN $4 THEN $5 ELSE crs_srid END,
                slug = COALESCE($6, slug),
                last_updated = NOW(),
                last_updated_by = $7
            WHERE id = $8
            RETURNING id
            "#,
            &dto.status as &Option<Status>,
            name,
            &dto.visibility as &Option<Visibility>,
            crs_srid_provided,
            crs_srid_value,
            slug,
            user_id.0,
            dto.id.0
        )
        .fetch_one(&mut *conn)
        .await?;

        Ok(ProjectId(result.id))
    }
}
