use domain::turbine_layout::{DuplicateTurbineInputDTO, TurbineLayoutInputDTO};
use domain::{LayoutId, ProjectId, UserId};
use sqlx::{Postgres, Transaction};

use crate::repo::RepositoryError;
use crate::repo::traits::Insert;

pub async fn insert_layout(
    tx: &mut Transaction<'_, Postgres>,
    layout: &TurbineLayoutInputDTO,
    user_id: UserId,
    project_id: ProjectId,
) -> Result<LayoutId, RepositoryError> {
    if layout.primary == Some(true) {
        sqlx::query!(
            "UPDATE app.turbine_layouts SET is_primary=false WHERE is_primary=true AND project_id=$1",
            project_id.0
        )
        .execute(&mut **tx)
        .await?;
    }
    let id = sqlx::query_scalar!(
        "INSERT INTO app.turbine_layouts(project_id, name, is_primary, added_by, last_updated_by)
         VALUES ($1, $2, COALESCE($3, NOT EXISTS(SELECT 1 FROM app.turbine_layouts WHERE project_id = $1 AND status = 'ACTIVE')), $4, $4)
         RETURNING id",
        project_id.0,
        layout.name.as_ref(),
        layout.primary,
        user_id.0,
    )
    .fetch_one(&mut **tx)
    .await?;
    Ok(LayoutId(id))
}

impl Insert for (&TurbineLayoutInputDTO, ProjectId, UserId) {
    type Id = LayoutId;

    async fn insert<'a, A>(&self, executor: A) -> Result<Self::Id, RepositoryError>
    where
        Self: Sized,
        A: sqlx::Acquire<'a, Database = sqlx::Postgres>,
    {
        let (dto, project_id, user_id) = self;
        let mut tx = executor.begin().await?;

        let layout_id = insert_layout(&mut tx, dto, *user_id, *project_id).await?;

        for turbine in dto.turbines.as_ref() {
            let wkb = turbine
                .geom
                .wkb()
                .map_err(|e| sqlx::Error::Protocol(e.to_string()))?;
            sqlx::query!(
                "INSERT INTO app.turbines(layout_id, turbine_number, hub_height_mm, rotor_diameter_mm, geom)
                 VALUES ($1, $2, $3, $4, ST_Transform(ST_GeomFromWKB($5, $6), $7::int))",
                layout_id.0,
                turbine.turbine_number as i32,
                turbine.hub_height_mm.map(|v| v as i32),
                turbine.rotor_diameter_mm.map(|v| v as i32),
                wkb,
                dto.srid,
                dto.target_srid,
            )
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        Ok(layout_id)
    }
}

impl Insert for (&DuplicateTurbineInputDTO, ProjectId, LayoutId, UserId) {
    type Id = LayoutId;

    async fn insert<'a, A>(&self, executor: A) -> Result<Self::Id, RepositoryError>
    where
        Self: Sized,
        A: sqlx::Acquire<'a, Database = Postgres>,
    {
        let (dto, project_id, source_layout_id, user_id) = self;
        let mut tx = executor.begin().await?;

        if dto.primary == Some(true) {
            sqlx::query!(
                "UPDATE app.turbine_layouts SET is_primary=false WHERE is_primary=true AND project_id=$1",
                project_id.0
            )
            .execute(&mut *tx)
            .await?;
        }

        let new_id = sqlx::query_scalar!(
            "INSERT INTO app.turbine_layouts(project_id, name, is_primary, added_by, last_updated_by)
             SELECT $1, COALESCE($2, tl.name), COALESCE($3, false), $4, $4
             FROM app.turbine_layouts tl
             WHERE tl.id = $5
               AND tl.project_id = $1
             RETURNING id",
            project_id.0,
            dto.name.as_ref().map(|n| n.as_ref()),
            dto.primary,
            user_id.0,
            source_layout_id.0,
        )
        .fetch_optional(&mut *tx)
        .await?
        .ok_or(RepositoryError::RowNotFound)?;

        sqlx::query!(
            "INSERT INTO app.turbines(layout_id, turbine_number, hub_height_mm, rotor_diameter_mm, geom)
             SELECT $1, t.turbine_number, COALESCE($2, t.hub_height_mm), COALESCE($3, t.rotor_diameter_mm), t.geom
             FROM app.turbines t
             WHERE t.layout_id = $4",
            new_id,
            dto.hub_height_mm,
            dto.rotor_diameter_mm,
            source_layout_id.0,
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(LayoutId(new_id))
    }
}
